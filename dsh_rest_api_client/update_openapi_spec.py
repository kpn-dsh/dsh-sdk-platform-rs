import json
import sys
import os

def generate_operation_id(path, method, tag):
    # Generate a meaningful operationId based on the tag, method and path
    path_parts = path.strip('/').split('/')
    # remove allocation from path
    path_parts = [part for part in path_parts if part != 'allocation']
    operation_id = tag.lower() + method.capitalize() + ''.join(part.capitalize() if part[0] != '{' else 'By' + part[1:-1].capitalize() for part in path_parts)
    return operation_id

def add_mising_items(openapi_spec):
    paths = openapi_spec.get('paths', {})
    for path, methods in paths.items():
        for method in ['get', 'post', 'put', 'delete', 'patch']:
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
    # Check if the input file exists
    try:
        with open(input_file, 'r') as file:
            openapi_spec = json.load(file)
    except FileNotFoundError:
        print(f"File not found: {input_file}")
        sys.exit(1)

    # Add operationId to each path
    updated_spec = add_mising_items(openapi_spec)

    api_version = updated_spec.get('info', {}).get('version', 'unknkown').replace('.', '_')
    dir_path = os.path.dirname(os.path.realpath(__file__))
    dir_path = os.path.join(dir_path, 'openapi_spec')
    output_file = os.path.join(dir_path, f'openapi_{api_version}.json')

    # Overwrite the input file with the updated spec
    with open(output_file, 'w') as file:
        json.dump(updated_spec, file, indent=2)

    print(f"Updated OpenAPI spec saved to {output_file}")


if __name__ == '__main__':
    # Read OpenAPI file from first argument
    if len(sys.argv) < 2:
        print("Usage: python3 update_openapi_spec.py <input_file>")
        sys.exit(1)
    input_file = sys.argv[1]
    update_spec(input_file)
    