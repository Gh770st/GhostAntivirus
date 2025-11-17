"""
GhostAntivirus AI Engine Setup Script
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read README file
this_directory = Path(__file__).parent
long_description = (this_directory / "README.md").read_text(encoding='utf-8') if (this_directory / "README.md").exists() else ""

# Read requirements
requirements = []
requirements_file = this_directory / "requirements.txt"
if requirements_file.exists():
    with open(requirements_file, 'r', encoding='utf-8') as f:
        requirements = [line.strip() for line in f if line.strip() and not line.startswith('#')]

setup(
    name="ghost-antivirus-ai-engine",
    version="3.0.0",
    author="WiterDevelopment",
    author_email="info@witerdev.com",
    description="AI-powered threat detection and analysis engine",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/witerdev/GhostAntivirus",
    packages=find_packages(where="src"),
    package_dir={"": "src"},
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Security",
        "Topic :: Scientific/Engineering :: Artificial Intelligence",
    ],
    python_requires=">=3.8",
    install_requires=requirements,
    extras_require={
        "dev": [
            "pytest>=7.4.3",
            "pytest-asyncio>=0.21.1",
            "pytest-cov>=4.1.0",
            "black>=23.11.0",
            "flake8>=6.1.0",
            "mypy>=1.7.1",
        ],
        "gpu": [
            "tensorflow-gpu>=2.14.0",
        ],
    },
    entry_points={
        "console_scripts": [
            "ghost-ai=ai_engine.main:main",
        ],
    },
    include_package_data=True,
    package_data={
        "ai_engine": [
            "models/*.h5",
            "models/*.pkl",
            "data/*.csv",
            "config/*.toml",
        ],
    },
    zip_safe=False,
    keywords="antivirus ai machine learning security malware detection",
    project_urls={
        "Bug Reports": "https://github.com/witerdev/GhostAntivirus/issues",
        "Source": "https://github.com/witerdev/GhostAntivirus",
        "Documentation": "https://ghostantivirus.readthedocs.io/",
    },
)