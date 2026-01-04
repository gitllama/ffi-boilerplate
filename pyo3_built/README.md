uv tool install maturin
<!-- maturin build --release -->
uv run maturin build --interpreter python --out . --release

python -m zipfile -e corepyo3-*.whl ./dist