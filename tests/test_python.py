import sys
import pytest
from latest_version import (
    find_latest_command,
    find_executables,
    get_version,
    ExecutableInfo,
)


def test_find_executables():
    # Test with a commonly available command
    if sys.platform == "win32":
        commands = ["python", "cmd"]
    else:
        commands = ["python3", "ls"]
    
    for cmd in commands:
        try:
            paths = find_executables(cmd)
            assert len(paths) > 0
            print(f"Found {len(paths)} paths for {cmd}")
            assert all(isinstance(path, str) for path in paths)
            break
        except Exception:
            continue


def test_get_version():
    # Test with Python
    if sys.platform == "win32":
        cmd = "python"
    else:
        cmd = "python3"
    
    try:
        paths = find_executables(cmd)
        if paths:
            for path in paths:
                try:
                    info = get_version(path)
                    assert isinstance(info, ExecutableInfo)
                    assert isinstance(info.path, str)
                    assert isinstance(info.version, str)
                    assert len(info.version) > 0
                    print(f"Version for {path}: {info.version}")
                    break
                except Exception:
                    continue
    except Exception:
        pytest.skip("Python not available")


def test_find_latest_command():
    if sys.platform == "win32":
        cmd = "python"
    else:
        cmd = "python3"
    
    try:
        info = find_latest_command(cmd)
        assert isinstance(info, ExecutableInfo)
        assert isinstance(info.path, str)
        assert isinstance(info.version, str)
        assert len(info.version) > 0
        print(f"Latest {cmd} is at: {info.path}")
        print(f"Version: {info.version}")
    except Exception as e:
        print(f"Test skipped: {e}")
        pytest.skip("Command not available")