import os
import subprocess

# Define the workflow content
workflow_content = """
name: Deploy to GitHub Pages

on:
  push:
    branches:
      - main  # Change this to your default branch if it's not 'main'

jobs:
  deploy:
    runs-on: ubuntu-latest

    steps:
    - name: Checkout repository
      uses: actions/checkout@v2

    - name: Set up Node.js
      uses: actions/setup-node@v2
      with:
        node-version: '14'

    - name: Install dependencies
      run: npm install

    - name: Build the project
      run: npm run build  # Adjust this if your build command is different

    - name: Deploy to GitHub Pages
      uses: peaceiris/actions-gh-pages@v3
      with:
        github_token: ${{ secrets.GITHUB_TOKEN }}
        publish_dir: ./dist  # Adjust this to the directory you want to publish
"""

# Define the directory and file paths
workflow_dir = os.path.join(".github", "workflows")
workflow_file = os.path.join(workflow_dir, "deploy.yml")

# Create the directories if they don't exist
os.makedirs(workflow_dir, exist_ok=True)

# Write the workflow content to the file
with open(workflow_file, "w") as file:
    file.write(workflow_content)


# Make .gitigore and add .env
with open(".gitignore", "w") as file:
    file.write(".env\n")

# Initialize a git repository if it doesn't exist
if not os.path.exists(".git"):
    subprocess.run(["git", "init"])

# Add the new files to git
subprocess.run(["git", "add", workflow_file, ".gitignore"])
subprocess.run(["git", "add", ".gitignore"])

# Commit the new files
subprocess.run(
    ["git", "commit", "-m", "Add GitHub Action for deploying to GitHub Pages"])

# Create a new repository on GitHub using the GitHub CLI
repo_name = input("Enter the repository name: ")
subprocess.run(["gh", "repo", "create", repo_name,
               "--public", "--source=.", "--remote=origin"])

# Push the changes to the remote repository
subprocess.run(["git", "push", "-u", "origin", "main"])

print("GitHub Action for deploying to GitHub Pages has been set up and pushed to the repository.")
