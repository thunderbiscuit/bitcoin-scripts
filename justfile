[group("Base")]
[doc("List all available commands.")]
@list:
  just --list --unsorted

[group("Base")]
[doc("Open repository on GitHub.")]
repo:
  open https://github.com/thunderbiscuit/bitcoin-scripts

[group("Running the Scripts")]
[doc("Run all Kotlin scripts")]
run-kotlin:
    for script in $(find kotlin -name "*.main.kts" | sort); do \
        echo ""; \
        echo "Running → $script"; \
        kotlinc -script "$script"; \
    done

[group("Running the Scripts")]
[doc("Run all Python scripts")]
run-python:
    for script in $(find python -name "*.py" | sort); do \
        echo ""; \
        echo "Running → $script"; \
        uv run "$script"; \
    done

[group("Running the Scripts")]
[doc("Run all Swift scripts")]
run-swift:
    for script in $(find swift -name "*.swift" | sort); do \
        echo ""; \
        echo "Running → $script"; \
        swift sh "$script"; \
    done
