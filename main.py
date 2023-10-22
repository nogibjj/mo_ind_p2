"""
Week two script
"""

from py_src.man_display_stat import descriptive_statistics
import psutil
import time


def display_main():
    csv_file_path = "datasets/cereal.csv"
    selected_columns = ["fat"]

    # Measure the start time
    start_time = time.time()

    # Measure the start memory usage
    process = psutil.Process()
    start_memory_usage = process.memory_percent()

    result = descriptive_statistics(csv_file_path, selected_columns)

    end_time = time.time()

    end_memory_usage = process.memory_percent()

    elapsed_time = end_time - start_time

    if result is not None:
        for col, std in result.items():
            print(f"{col} Standard Deviation: {std:.4f}")
    print(f"Memory Usage: {end_memory_usage:.6f}%")
    print(f"Elapsed time: {elapsed_time:.6f} seconds")


if __name__ == "__main__":
    display_main()
