#!/usr/bin/env python3
"""Summarize saved convergence experiments without recomputing them."""
import argparse
import json
from pathlib import Path

STATUSES = ('PASS', 'FAIL', 'INCOMPLETE', 'UNASSESSED', 'UNSUPPORTED')


def validate_checks(checks):
    if not isinstance(checks, list):
        raise ValueError('checks must be an array')
    for check in checks:
        if not isinstance(check, dict) or not all(
                isinstance(check.get(field), str) for field in ('name', 'category', 'status', 'reason')):
            raise ValueError('each check must contain name, category, status, and reason strings')
        if check['status'] not in STATUSES:
            raise ValueError('unknown check status: ' + check['status'])


def assess(root):
    runs = sorted(Path(root).glob('run-*'))
    records = []
    complete = bool(runs)
    counts = dict.fromkeys(STATUSES, 0)
    for run in runs:
        record = dict(run=str(run), measurements=None)
        records.append(record)
        errors = []
        # Preserve each available artifact even when another stage never ran.
        for name in ('summary', 'request', 'build', 'failure'):
            path = run / (name + '.json')
            if name == 'failure' and not path.exists():
                continue
            try:
                record[name] = json.loads(path.read_text(encoding='utf-8'))
                if not isinstance(record[name], dict):
                    raise ValueError('expected a JSON object')
            except (OSError, ValueError) as error:
                errors.append(f'{path.name}: {error}')
        failure = record.get('failure')
        if isinstance(failure, dict) and failure.get('error'):
            errors.append('required computation failed: ' + str(failure['error']))
        try:
            summary = record['summary']
            if summary['schema_version'] != 1:
                raise ValueError('unsupported summary schema')
            if not isinstance(summary['measurements_saved'], bool):
                raise ValueError('measurements_saved must be a boolean')
            validate_checks(summary['checks'])
            integrity = summary['run_status']
            if integrity not in ('PASS', 'INCOMPLETE'):
                raise ValueError('unknown run integrity status: ' + str(integrity))
            print(f'[{integrity}] recorded run integrity: {run.name}')
            print('Applicable capture completeness: ' + str(summary.get('capture_status', 'UNKNOWN')))
            for check in summary['checks']:
                counts[check['status']] += 1
                print(f"[{check['status']}] {check['category']}: {check['name']} - {check['reason']}")
            if not summary['measurements_saved']:
                errors.append('measurements were not produced; see the recorded computation failure and checks')
            complete &= integrity == 'PASS'
        except (ValueError, KeyError, TypeError) as error:
            errors.append(f'summary.json: {error}')
        path = run / 'measurements.json'
        if path.exists():
            try:
                measurements = json.loads(path.read_text(encoding='utf-8'))
                record['measurements'] = measurements
                if not isinstance(measurements, dict):
                    raise ValueError('expected a JSON object')
                if not isinstance(measurements.get('observable'), str) or 'values' not in measurements:
                    raise ValueError('expected an observable string and values field')
                validate_checks(measurements.get('checks'))
            except (OSError, ValueError) as error:
                errors.append(f'{path.name}: {error}')
        elif isinstance(record.get('summary'), dict) and record['summary'].get('measurements_saved'):
            errors.append('measurements.json is missing despite measurements_saved=true')
        if errors:
            complete = False
            counts['INCOMPLETE'] += 1
            record['error'] = '; '.join(errors)
            for error in errors:
                print(f'[INCOMPLETE] {run.name}: {error}')
    if not runs:
        print('[INCOMPLETE] no run journals found')
        counts['INCOMPLETE'] += 1
    print('Checks: ' + ', '.join(f'{k}={v}' for k, v in counts.items()))
    print('Overall run integrity: ' + ('PASS' if complete else 'INCOMPLETE'))
    print('FAIL hypothesis means a negative finite result; it is not an execution failure.')
    print('UNASSESSED means the evidence does not decide the statement; no asymptotic proof is inferred.')
    return dict(schema_version=1, complete=complete, check_counts=counts, runs=records)

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run_root', type=Path)
    parser.add_argument('--output', type=Path, help='new aggregate JSON file; never overwrites')
    args = parser.parse_args()
    report = assess(args.run_root)
    if args.output:
        with args.output.open('x', encoding='utf-8') as stream:
            json.dump(report, stream, indent=2)
            stream.write('\n')
    return 0 if report['complete'] else 1

if __name__ == '__main__':
    raise SystemExit(main())
