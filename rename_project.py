import os
import re

renames = {
    r'(?i)izboss': 'iZBoss',
    r'(?i)izcore': 'iZCore',
    r'izfxTrade/izboss': 'iZFxTrade/iZBoss',
    r'iZFxTrade/izboss': 'iZFxTrade/iZBoss'
}

root_dir = '/Users/mgn63opt/Desktop/boss.iz.life'
exclude_dirs = {'.git', 'target', '.gemini'}

for root, dirs, files in os.walk(root_dir):
    dirs[:] = [d for d in dirs if d not in exclude_dirs]
    for file in files:
        if file.endswith(('.md', '.sh', '.ps1', '.rs', '.ts', '.toml', '.sql')):
            file_path = os.path.join(root, file)
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read()
            
            new_content = content
            for pattern, replacement in renames.items():
                # For URLs and specific paths, we might need more precision, 
                # but the user requested general renaming.
                new_content = re.sub(pattern, replacement, new_content)
            
            if new_content != content:
                with open(file_path, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"Updated: {file_path}")
