import os

version = (
    os.environ['VERSION'][1:] 
    if os.environ['VERSION'].startswith('v') 
    else os.environ['VERSION']
)

original_content = None
with open('Cargo.toml', 'r') as f:
    original_content= f.read()
    if original_content is None:
        raise ValueError("expected to read Cargo.toml")

content_with_updated_version = original_content.replace("0.0.1-dirty", version)
with open('Cargo.toml', 'w') as w:
    w.write(content_with_updated_version)

