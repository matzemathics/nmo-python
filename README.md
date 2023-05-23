# nmo-python - Python bindings for nemo

example usage:
```python
from nmo_python import load_string, NemoEngine

rules="""
data(1,2) .
result(?x, !v) :- data(?y, ?x) .
"""

engine = NemoEngine(load_string(rules))
predicates = engine.reason()

for pred in predicates:
  print(pred.name())
  print(engine.result(pred))
```
