from setuptools import setup, find_packages

setup(
    name="polygone-tui",
    version="1.0.0",
    packages=find_packages(),
    install_requires=[
        "textual>=0.80.0",
        "rich>=13.0.0",
        "click>=8.0.0",
    ],
    entry_points={
        "console_scripts": [
            "polygone=polygone_tui.cli:main",
        ],
    },
    author="Manus x Polygone",
    description="Une interface terminal simplifiée pour le projet Polygone",
    long_description=open("README.md").read() if open("README.md") else "",
    long_description_content_type="text/markdown",
    url="https://github.com/lvs0/Polygone",
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.8",
)
