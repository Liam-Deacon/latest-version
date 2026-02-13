import argparse
import sys

from latest_version import find_latest_command


def main():
    parser = argparse.ArgumentParser(
        description="Find the latest version of commands across all available paths"
    )
    parser.add_argument("command", help="Command to check for latest version")
    args = parser.parse_args()

    try:
        result = find_latest_command(args.command)
        print(result.path)
        return 0
    except Exception as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())