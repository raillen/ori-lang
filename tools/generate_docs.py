import os
import glob
import re

zdoc_dir = 'stdlib/zdoc/std'
out_dir = 'docs/public/stdlib'

def parse_zdoc(content):
    targets = []
    # Split by `--- @target: `
    parts = re.split(r'^---\s*@target:\s*(\w+)\s*$', content, flags=re.MULTILINE)
    
    # First part is usually empty or general top-level stuff before the first target
    if len(parts) > 0 and parts[0].strip() == '':
        parts = parts[1:]
        
    for i in range(0, len(parts), 2):
        if i + 1 < len(parts):
            target_name = parts[i].strip()
            target_content = parts[i+1].strip()
            
            # Remove trailing `---` if present
            target_content = re.sub(r'\n---$', '', target_content).strip()
            targets.append({
                'name': target_name,
                'content': target_content
            })
    return targets

def generate_md(zdoc_path, rel_path):
    with open(zdoc_path, 'r', encoding='utf-8') as f:
        content = f.read()
        
    targets = parse_zdoc(content)
    
    # Create module name
    module_name = rel_path.replace('\\', '/').replace('.zdoc', '')
    module_name = 'std.' + module_name.replace('/', '.')
    
    md_content = f"# Modulo `{module_name}`\n\n"
    
    # Find namespace target first
    namespace_target = next((t for t in targets if t['name'] == 'namespace'), None)
    if namespace_target:
        # The content of namespace usually has `# std...` at the top, let's remove the first line if it's a heading matching the module
        lines = namespace_target['content'].split('\n')
        if lines and lines[0].startswith('#'):
            lines = lines[1:]
        
        desc = '\n'.join(lines).strip()
        md_content += f"{desc}\n\n"
    
    md_content += "## Constantes e Funcoes\n\n"
    
    for target in targets:
        if target['name'] == 'namespace':
            continue
            
        md_content += f"### `{target['name']}`\n\n"
        md_content += f"{target['content']}\n\n"
        
    # Write to output directory
    out_file = os.path.join(out_dir, rel_path.replace('.zdoc', '.md'))
    os.makedirs(os.path.dirname(out_file), exist_ok=True)
    
    with open(out_file, 'w', encoding='utf-8') as f:
        f.write(md_content)
    print(f"Generated {out_file}")

def main():
    os.makedirs(out_dir, exist_ok=True)
    
    # Find all .zdoc files
    pattern = os.path.join(zdoc_dir, '**/*.zdoc')
    zdoc_files = glob.glob(pattern, recursive=True)
    
    for zdoc_file in zdoc_files:
        rel_path = os.path.relpath(zdoc_file, zdoc_dir)
        generate_md(zdoc_file, rel_path)
        
    print("Documentation generation complete.")

if __name__ == '__main__':
    main()
