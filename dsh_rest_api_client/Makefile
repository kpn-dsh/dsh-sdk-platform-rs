
.PHONY: build check-spec update-spec generate postprocess install fmt clean-tmp

SPEC ?=
SPEC_PATH := openapi_spec/$(SPEC)
UPDATED_SPEC_MARKER := .generated_openapi_spec_path
TMP_DIR := tmp
TMP_LIB_RS := $(TMP_DIR)/src/lib.rs
TARGET_LIB_RS := src/lib.rs
POSTPROCESS_SCRIPT := postprocess_generated_lib_rs.py

build: install fmt

check-spec:
	@test -n "$(SPEC)" || { echo "Usage: make build SPEC=<openapi-spec-file>"; exit 1; }
	@test -f "$(SPEC_PATH)" || { echo "Spec file not found: $(SPEC_PATH)"; exit 1; }

update-spec: check-spec
	@updated_spec="$$(python3 update_openapi_spec.py "./$(SPEC_PATH)" | sed -n 's/^Updated OpenAPI spec saved to //p')"; \
	if [ -z "$$updated_spec" ]; then \
		echo "Failed to determine updated OpenAPI spec path"; \
		exit 1; \
	fi; \
	if [ ! -f "$$updated_spec" ]; then \
		echo "Updated OpenAPI spec not found: $$updated_spec"; \
		exit 1; \
	fi; \
	printf '%s\n' "$$updated_spec" > "$(UPDATED_SPEC_MARKER)"

generate: update-spec
	@updated_spec="$$(cat "$(UPDATED_SPEC_MARKER)")"; \
	rm -rf "$(TMP_DIR)"; \
	cargo +nightly progenitor -i "$$updated_spec" -o "$(TMP_DIR)" -n dsh_rest_client -v 0.1.0

postprocess: generate
	@python3 "$(POSTPROCESS_SCRIPT)" "$(TMP_LIB_RS)"

install: postprocess
	@mv "$(TMP_LIB_RS)" "$(TARGET_LIB_RS)"
	@rm -rf "$(TMP_DIR)" "$(UPDATED_SPEC_MARKER)"

fmt:
	@cargo fmt

clean-tmp:
	@rm -rf "$(TMP_DIR)" "$(UPDATED_SPEC_MARKER)"
