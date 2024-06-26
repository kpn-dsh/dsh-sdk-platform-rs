import json
import sys

def generate_operation_id(path, method):
    # Generate a meaningful operationId based on the path and method
    path_parts = path.strip('/').split('/')
    operation_id = method.lower() + ''.join(part.capitalize() if part[0] != '{' else 'By' + part[1:-1].capitalize() for part in path_parts)
    return operation_id

def add_mising_items(openapi_spec):
    paths = openapi_spec.get('paths', {})
    for path, methods in paths.items():
        for method in ['get', 'post', 'put', 'delete', 'patch']:
            if method in methods:
                # Add operationId if missing
                if 'operationId' not in methods[method]:
                    operation_id = generate_operation_id(path, method)
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

    # Overwrite the input file with the updated spec
    with open(input_file, 'w') as file:
        json.dump(updated_spec, file, indent=2)

    print(f"Updated OpenAPI spec saved to {input_file}")


if __name__ == '__main__':
    # Read OpenAPI file from first argument
    if len(sys.argv) < 2:
        print("Usage: python3 update_openapi_spec.py <input_file>")
        sys.exit(1)
    input_file = sys.argv[1]
    update_spec(input_file)
    