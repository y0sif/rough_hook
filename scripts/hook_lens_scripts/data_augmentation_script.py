import os
from PIL import Image, ImageEnhance, ImageOps
import numpy as np
import random

# Define the paths
train_dir = 'val_resize'
output_dir = 'augmented_val'

# Create the output directory if it doesn't exist
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

# Function to add noise to an image
def add_noise(image):
    np_image = np.array(image)
    noise = np.random.randint(0, 50, np_image.shape, dtype='uint8')
    noisy_image = np.clip(np_image + noise, 0, 255)
    return Image.fromarray(noisy_image)

# Function to rotate an image by 45 degrees
def rotate_image_45(image):
    return image.rotate(45, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_90(image):
    return image.rotate(90, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_135(image):
    return image.rotate(135, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_180(image):
    return image.rotate(180, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_225(image):
    return image.rotate(225, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_270(image):
    return image.rotate(270, expand=True).resize((32, 32))

# Function to rotate an image by 45 degrees
def rotate_image_315(image):
    return image.rotate(315, expand=True).resize((32, 32))

# Function to flip an image horizontally
def flip_image(image):
    return ImageOps.mirror(image)

# Function to adjust brightness
def adjust_brightness(image):
    enhancer = ImageEnhance.Brightness(image)
    return enhancer.enhance(random.uniform(0.7, 1.3))

# Function to adjust contrast
def adjust_contrast(image):
    enhancer = ImageEnhance.Contrast(image)
    return enhancer.enhance(random.uniform(0.7, 1.3))

# Function to perform augmentation on a single image
def augment_image(image_path, output_path):
    image = Image.open(image_path)
    
    # Original image
    image.save(output_path)
    
    # Add noise
    noisy_image = add_noise(image)
    noisy_image.save(output_path.replace('.jpg', '_noise.jpg'))
    
    # Rotate image
    rotated_image = rotate_image_45(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_45.jpg'))
    

    # Rotate image
    rotated_image = rotate_image_90(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_90.jpg'))
    noisy_image = add_noise(rotated_image)
    noisy_image.save(output_path.replace('.jpg', '_rotated_90_noise.jpg'))

    # Rotate image
    rotated_image = rotate_image_135(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_135.jpg'))
   
    
    # Rotate image
    rotated_image = rotate_image_180(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_180.jpg'))
    noisy_image = add_noise(rotated_image)
    noisy_image.save(output_path.replace('.jpg', '_rotated_180_noise.jpg'))
    
    # Rotate image
    rotated_image = rotate_image_225(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_225.jpg'))
    
    
    # Rotate image
    rotated_image = rotate_image_270(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_270.jpg'))
    noisy_image = add_noise(rotated_image)
    noisy_image.save(output_path.replace('.jpg', '_rotated_270_noise.jpg'))
    
    # Rotate image
    rotated_image = rotate_image_315(image)
    rotated_image.save(output_path.replace('.jpg', '_rotated_315.jpg'))
    
    
    # Flip image
    flipped_image = flip_image(image)
    flipped_image.save(output_path.replace('.jpg', '_flipped.jpg'))
   
    
    # Adjust brightness
    bright_image = adjust_brightness(image)
    bright_image.save(output_path.replace('.jpg', '_bright.jpg'))
    
    
    # Adjust contrast
    contrast_image = adjust_contrast(image)
    contrast_image.save(output_path.replace('.jpg', '_contrast.jpg'))
   

# Walk through the train directory and augment images
for root, dirs, files in os.walk(train_dir):
    for file in files:
        if file.endswith('.jpg'):
            image_path = os.path.join(root, file)
            relative_path = os.path.relpath(root, train_dir)
            output_subdir = os.path.join(output_dir, relative_path)
            
            if not os.path.exists(output_subdir):
                os.makedirs(output_subdir)
            
            output_image_path = os.path.join(output_subdir, file)
            augment_image(image_path, output_image_path)

print("Augmentation complete!")