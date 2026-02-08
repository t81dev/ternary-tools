# Phase 2 Inspection/Debug Integration Checklist

Roadmap tracker: https://github.com/t81dev/t81-roadmap/issues/15

This checklist defines the first handoff path between pager diagnostics output and
`ternary-tools` inspection workflows.

## Scope

- Input source: `ternary-pager/benchmarks/parse_fault_output.py` CSV output.
- Goal: quickly inspect decode/fault behavior from a CLI-friendly path.
- Non-goal: replacing pager benchmark orchestration.

## Checklist

- [x] Define shared artifact format
  - Contract: CSV headers include `pattern`, `faults`, `decodes`,
    `bytes_decoded`, `bytes_encoded`, and `avg_decode_us`.
- [x] Identify first script/tool touchpoint
  - Touchpoint command:
    `python3 ../ternary-pager/benchmarks/parse_fault_output.py --input fault.log --output fault.csv`
  - Follow-up inspection path:
    `ternary-tools gguf summary model.gguf --ternary` plus CSV metrics review for
    runtime decode/fault context.
- [x] Define ownership boundary
  - `ternary-pager`: produces benchmark logs + CSV.
  - `ternary-tools`: consumes metrics in operator workflow documentation.
- [x] Record rollout gate
  - Gate: at least one captured CSV per workload profile (sequential, random,
    strided) is archived before adding automated ingestion.

## Next increments

1. Add `ternary-tools` subcommand proposal for pager CSV inspection.
2. Add fixture CSV in `ternary-tools` for parser contract tests.
3. Add CI check that fails on missing required CSV headers.
