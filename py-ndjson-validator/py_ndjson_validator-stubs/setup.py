from setuptools import setup

setup(
    name="py-ndjson-validator-stubs",
    version="0.1.0",
    description="Type stubs for py-ndjson-validator",
    packages=["py_ndjson_validator-stubs"],
    python_requires=">=3.10",
    install_requires=["py-ndjson-validator>=0.1.0"],
    package_data={"py_ndjson_validator-stubs": ["__init__.pyi"]},
)
