#!/usr/bin/env python3
import subprocess
import json
import sys
import numpy as np
import matplotlib.pyplot as plt
import argparse


def do_gather_data(stream):
    result = {
        'perft': {},
        'hperft': {},
    }
    for ln in stream:
        data = json.loads(ln)
        if data['reason'] != 'benchmark-complete':
            continue
        suite, case, impl = data['id'].split('/')
        val = data['mean']['estimate']
        units = data['mean']['unit']
        assert units == 'ns'
        val /= 1_000_000.0
        result[suite].setdefault(impl, {})[case] = val
    return result


def gather_data():
    try:
        p = subprocess.Popen(
            ['cargo', 'criterion', '--message-format', 'json'],
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            text=True,
        )
        return do_gather_data(p.stdout)
    finally:
        code = p.poll()
        if code is not None and code != 0:
            raise RuntimeError(f'process exited with code {code}')
        p.kill()


def build_plots(data, show):
    for sname, sub in data.items():
        impls = sorted([(k, v) for k, v in sub.items()])
        labels = sorted(impls[0][1])
        y = np.arange(len(labels))
        fig, ax = plt.subplots(figsize=(8, len(labels)))
        height = 0.8
        for idx, (iname, cases) in enumerate(impls):
            assert labels == sorted(cases)
            cases = sorted([(k, v) for k, v in cases.items()])
            vals = [c[1] for c in cases]
            rects = ax.barh(
                y - height / 2 + idx * height / len(impls),
                vals,
                height / len(impls),
                label=iname,
            )
            ax.bar_label(rects, padding=3)
        ax.set_xlabel('time, ms')
        ax.set_title(sname)
        ax.set_yticks(y, labels)
        ax.legend()
        fig.tight_layout()
        fig.savefig(f'{sname}.svg')
    if show:
        plt.show()


parser = argparse.ArgumentParser(
    description='Run the benchmarks for all the chess implementations.')
parser.add_argument('-d', '--data-file', type=argparse.FileType('r'),
                    action='store',
                    help='file with benchmark results (if not specified, ' +
                         'run the benchmarks to obtain the results)')
parser.add_argument('-o', '--output-file', type=argparse.FileType('w'),
                    action='store',
                    help='file to write the benchmark results')
parser.add_argument('-s', '--show', action='store_true', help='store plots')
args = parser.parse_args()

if args.data_file is not None:
    data = json.load(args.data_file)
else:
    data = gather_data()
if args.output_file is not None:
    json.dump(data, args.output_file, indent=2)

build_plots(data, args.show)
