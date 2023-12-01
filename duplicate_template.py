# written by my bff copilot

import os

# Define the path to the directory where the files are located
dir_path = "src/problems/"

# Read the original file
with open(os.path.join(dir_path, "dtemplate.rs"), "r") as file:
    file_data = file.read()

# Loop over the range you want to create files for
for i in range(1, 26):  # Change this range as needed
    # Replace the DAY constant with the current number
    new_file_data = file_data.replace('const DAY: u32 = 1;', f'const DAY: u32 = {i};')

    # Write the new file
    with open(os.path.join(dir_path, f"d{i:02}.rs"), "w") as file:
        file.write(new_file_data)

