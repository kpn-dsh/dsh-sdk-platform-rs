import json
import yaml
import sys
import os

def generate_operation_id(path, method, tag):
    # Generate a meaningful operationId based on the tag, method and path
    path_parts = path.strip('/').split('/')
    # remove 'allocation' from path
    path_parts = [part for part in path_parts if part != 'allocation']
    # split tag by space and capitalize each word (except the first one)
    tag = tag.split(' ')
    tag = ''.join([tag[0]] + [word.capitalize() for word in tag[1:]])
    # Create operationId by combining tag, method, and path parts
    operation_id = tag + method.capitalize() + ''.join(
        part.capitalize() if part[0] != '{' else 'By' + part[1:-1].capitalize() for part in path_parts
    )
    return operation_id

def add_missing_items(openapi_spec):
    paths = openapi_spec.get('paths', {})
    for path, methods in paths.items():
        for method in ['get', 'post', 'put', 'delete', 'patch', 'head']:
            if method in methods:
                tag = methods[method].get('tags', [''])[0]
                # Add operationId if missing
                if 'operationId' not in methods[method]:
                    operation_id = generate_operation_id(path, method, tag)
                    methods[method]['operationId'] = operation_id
                
                # Append bearer auth parameters
                if 'parameters' not in methods[method]:
                    methods[method]['parameters'] = []
                methods[method]['parameters'].append({
                    "name": "Authorization",
                    "in": "header",
                    "description": "Authorization header (bearer token)",
                    "required": True,
                    "deprecated": False,
                    "schema": {
                        "type": "string"
                    }
                })
    return openapi_spec

def update_spec(input_file):
    # Determine if file is JSON or YAML
    file_extension = os.path.splitext(input_file)[1].lower()
    
    try:
        with open(input_file, 'r') as file:
            if file_extension == '.json':
                openapi_spec = json.load(file)
            elif file_extension in ['.yaml', '.yml']:
                openapi_spec = yaml.safe_load(file)
            else:
                print("Unsupported file format. Please use JSON or YAML.")
                sys.exit(1)
    except FileNotFoundError:
        print(f"File not found: {input_file}")
        sys.exit(1)

    # Add operationId and other items to each path
    updated_spec = add_missing_items(openapi_spec)

    api_version = updated_spec.get('info', {}).get('version', 'unknown').replace('.', '_')
    dir_path = os.path.dirname(os.path.realpath(__file__))
    dir_path = os.path.join(dir_path, 'openapi_spec')
    os.makedirs(dir_path, exist_ok=True)

    # Set output file based on original format
    output_file = os.path.join(dir_path, f'openapi_{api_version}.{file_extension.strip(".")}')
    
    # Save the updated spec in the appropriate format
    with open(output_file, 'w') as file:
        if file_extension == '.json':
            json.dump(updated_spec, file, indent=2)
        else:
            yaml.dump(updated_spec, file, default_flow_style=False)

    print(f"Updated OpenAPI spec saved to {output_file}")

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: python3 update_openapi_spec.py <input_file>")
        sys.exit(1)
    input_file = sys.argv[1]
    update_spec(input_file)
