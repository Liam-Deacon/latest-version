"""
Latest Version Finder

Find the latest version of commands across all available paths.
"""

from latest_version._latest_version import (
    find_executables_py as find_executables,
    get_version_py as get_version,
    find_latest_command_py as find_latest_command,
    PyExecutableInfo as ExecutableInfo,
)

__version__ = "0.1.0"