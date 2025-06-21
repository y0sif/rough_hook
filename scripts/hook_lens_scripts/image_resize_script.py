import os
from PIL import Image

# Define source and destination paths
source_path = "val"
destination_path = "val_resize"
new_size = (32, 32)

# Create destination folder if it doesn't exist
os.makedirs(destination_path, exist_ok=True)

# Iterate through each folder and file in the source path
for root, dirs, files in os.walk(source_path):
    # Calculate the relative path
    relative_path = os.path.relpath(root, source_path)
    # Create the corresponding directory in the destination path
    target_dir = os.path.join(destination_path, relative_path)
    os.makedirs(target_dir, exist_ok=True)
    
    for file in files:
        # Full path of the source image
        source_file = os.path.join(root, file)
        # Full path for the resized image in the destination
        destination_file = os.path.join(target_dir, file)
        
        try:
            # Open and resize the image
            with Image.open(source_file) as img:
                resized_img = img.resize(new_size, Image.ANTIALIAS)
                # Save the resized image to the destination
                resized_img.save(destination_file)
            print(f"Resized and saved: {destination_file}")
        except Exception as e:
            print(f"Error processing {source_file}: {e}")





 