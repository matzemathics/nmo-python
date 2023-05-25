# nmo-python - Python bindings for nemo

example usage:
```python
from nmo_python import load_string, NemoEngine, NemoOutputManager

rules="""
data(1,2) .
data(hi,42) .
data(hello,world) .

calculated(?x, !v) :- data(?y, ?x) .
"""

engine = NemoEngine(load_string(rules))
engine.reason()

print(list(engine.result("calculated")))

output_manager = NemoOutputManager("results", gzip=True)
engine.write_result("calculated", output_manager)
```
