# nmo-python - Python bindings for nemo

> **Note**
> This repository has been migrated to [knowsys/nemo](https://github.com/knowsys/nemo)

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

## Known limitations

Currently `any` values are not correctly transformed into python types. This will be fixed once a missing feature in nemo (knowsys/nemo#245) is implemented.
