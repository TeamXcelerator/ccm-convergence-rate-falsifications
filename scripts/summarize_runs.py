#!/usr/bin/env python3
"""Summarize saved convergence experiments without recomputing them."""
import argparse
import json
from pathlib import Path

def assess(root):
    runs = sorted(Path(root).glob('run-*'))
    records = []
    complete = bool(runs)
    counts = dict.fromkeys(['PASS', 'FAIL', 'INCOMPLETE', 'UNASSESSED', 'UNSUPPORTED'], 0)
    for run in runs:
        try:
            summary = json.loads((run / 'summary.json').read_text())
            measurements = json.loads((run / 'measurements.json').read_text())
            request = json.loads((run / 'request.json').read_text())
            build = json.loads((run / 'build.json').read_text())
            if summary['schema_version'] != 1 or not summary['measurements_saved']:
                raise ValueError('missing measured evidence')
            integrity = summary['run_status']
            complete &= integrity == 'PASS'
            print(f'[{integrity}] run integrity: {run.name}')
            for check in summary['checks']:
                counts[check['status']] += 1
                print(f"[{check['status']}] {check['category']}: {check['name']} — {check['reason']}")
            records.append(dict(run=str(run), request=request, build=build,
                                summary=summary, measurements=measurements))
        except (OSError, ValueError, KeyError, TypeError) as error:
            complete = False
            counts['INCOMPLETE'] += 1
            print(f'[INCOMPLETE] {run}: {error}')
            records.append(dict(run=str(run), error=str(error)))
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
