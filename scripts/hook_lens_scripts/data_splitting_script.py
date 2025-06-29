import os
import shutil
import random

# Define paths
data_dir = 'data'
train_dir = 'train'
test_dir = 'test'
val_dir = 'val'

# Create train, test, val directories if they don't exist
os.makedirs(train_dir, exist_ok=True)
os.makedirs(test_dir, exist_ok=True)
os.makedirs(val_dir, exist_ok=True)

# Define split ratios
train_ratio = 0.6
test_ratio = 0.2
val_ratio = 0.2

# Iterate through each sub-folder in the data directory
for sub_folder in os.listdir(data_dir):
    sub_folder_path = os.path.join(data_dir, sub_folder)
    
    # Skip if it's not a directory
    if not os.path.isdir(sub_folder_path):
        continue
    
    # Get list of images in the sub-folder
    images = [img for img in os.listdir(sub_folder_path) if img.endswith(('.png', '.jpg', '.jpeg'))]
    random.shuffle(images)  # Shuffle the images to ensure randomness
    
    # Calculate the number of images for each split
    num_images = len(images)
    num_train = int(train_ratio * num_images)
    num_test = int(test_ratio * num_images)
    num_val = num_images - num_train - num_test
    
    # Split the images
    train_images = images[:num_train]
    test_images = images[num_train:num_train + num_test]
    val_images = images[num_train + num_test:]
    
    # Create sub-folders in train, test, val directories
    train_sub_folder = os.path.join(train_dir, sub_folder)
    test_sub_folder = os.path.join(test_dir, sub_folder)
    val_sub_folder = os.path.join(val_dir, sub_folder)
    
    os.makedirs(train_sub_folder, exist_ok=True)
    os.makedirs(test_sub_folder, exist_ok=True)
    os.makedirs(val_sub_folder, exist_ok=True)
    
    # Move images to their respective directories
    for img in train_images:
        shutil.move(os.path.join(sub_folder_path, img), os.path.join(train_sub_folder, img))
    
    for img in test_images:
        shutil.move(os.path.join(sub_folder_path, img), os.path.join(test_sub_folder, img))
    
    for img in val_images:
        shutil.move(os.path.join(sub_folder_path, img), os.path.join(val_sub_folder, img))

print("Dataset split completed successfully!")