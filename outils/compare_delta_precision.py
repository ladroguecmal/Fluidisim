"""S231 : mesurer le même banc de coût sur un candidat Git et le candidat courant.

Les sources figées et exécutables de mesure restent dans code/target, jamais dans le cœur.
Précondition : cargo build -p water-core --release --offline, rustc disponible.
"""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]
CODE = ROOT / "code"
TARGET = CODE / "target" / "s231-cost"
REFERENCE = "88b98fe"


def run(*args):
    return subprocess.run(args, cwd=ROOT, check=True, text=True, encoding="utf-8", capture_output=True).stdout


def main():
    deps = CODE / "target" / "release" / "deps"
    library = max(deps.glob("libwater_core-*.rlib"), key=lambda p: p.stat().st_mtime_ns)
    # Banc identique, imprimant médiane/max et qualité ; indépendamment du type de pression.
    driver = (CODE / "water-core" / "examples" / "block_cost.rs").read_text(encoding="utf-8")
    driver = driver.replace('#[path = "../../water-harness/src/host_impl.rs"]',
                            f'#[path = "{(CODE / "water-harness/src/host_impl.rs").as_posix()}"]')
    driver = driver.replace('    delta_projection::{Domain, Volume},\n', '')
    driver = ('pub use water_core::host;\nmod delta_projection;\n'
              'use delta_projection::{Domain, Volume};\n' + driver.replace('//!', '//'))
    for label, revision in [("f64", REFERENCE), ("f32", None)]:
        directory = TARGET / label
        directory.mkdir(parents=True, exist_ok=True)
        for name in ["delta_projection.rs", "delta_budget.rs"]:
            relative = f"code/water-core/src/{name}"
            source = run("git", "show", f"{revision}:{relative}") if revision else (ROOT / relative).read_text(encoding="utf-8")
            (directory / name).write_text(source, encoding="utf-8")
        source = directory / "main.rs"
        source.write_text(driver, encoding="utf-8")
        executable = directory / "cost.exe"
        run("rustc", "--edition=2021", "-O", "-C", "overflow-checks=yes", "-A", "warnings",
            "--extern", f"water_core={library}", "-L", f"dependency={deps}", str(source), "-o", str(executable))
        result = run(str(executable))
        (directory / "result.txt").write_text(result, encoding="utf-8")
        print(f"=== {label}, source={revision or 'working tree'}, même banc block_cost ===")
        print(result)


if __name__ == "__main__":
    main()
