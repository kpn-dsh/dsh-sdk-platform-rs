# Please edit the following variables to match your project
ifdef DSH_TENANT_NAME
    TENANT:=$(DSH_TENANT_NAME) # grab from environment variable
else
    TENANT:=
endif
ifdef DSH_TENANT_UID
    UID:=$(DSH_TENANT_UID) # grab from environment variable
else
    UID:=
endif
# You can change the following variables, but it is not necessary as it will ask during runtime
# NOTE: it's not recommended to store CLIENT_SECRET in the makefile!
PLATFORM ?= $(shell bash -c 'read -p "Platform (dsh-dev or poc-dsh): " PLATFORM; echo $$PLATFORM')
TOPIC ?= $(shell bash -c 'read -p "Topic to read: " TOPIC; echo $$TOPIC')
CLIENT_ID ?= $(shell bash -c 'read -p "Rest API client ID: " CLIENT_ID; echo $$CLIENT_ID')
CLIENT_SECRET ?= $(shell bash -c 'read -s -p "Rest API client secret: " CLIENT_SECRET; echo $$CLIENT_SECRET')

# DO NOT EDIT BELOW THIS LINE
DOCKER_REPO_URL=registry.cp.kpn-dsh.com/$(strip $(TENANT))
VERSION=example
TAGNAME=dsh-sdk-example
IMAGE=$(DOCKER_REPO_URL)/$(TAGNAME):$(VERSION)

verify_var:
	@if [ -z "$(TENANT)" ]; then echo "DSH_TENANT_NAME is not set in environment variable. Please set this as variable or add it to the makefile."; exit 1; fi
	@if [ -z "$(UID)" ]; then echo "DSH_TENANT_UID is not set in environment variable. Please set this as variable or add it to the makefile."; exit 1; fi
help:
	@echo "login		- login to the relevant repository"
	@echo "fix		- run dos2unix on every file"
	@echo "build		- build the image"
	@echo "rebuild		- build the image with the --no-cache flag"
	@echo "no-docker-build	- Cargo build on local machine"
	@echo "deploy		- deploy the image to DSH (requires Rest Credentials, see https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/$(TENANT)/rest-api)"
	@echo "all		- shorthand for fix, build, push, show"
	@echo "run		- run the image in local docker"
	@echo "push		- push the  image to jfrog"
	@echo "show		- show the current make variables"
	@echo 
	@echo "Currently only supports to deploy to dsh-dev and poc-dsh on AWS"

login:
	docker login $(DOCKER_REPO_URL) 
fix: 
	find . -type f -print0 | xargs -0 dos2unix
build: verify_var 
	docker build -t $(TAGNAME) -f Dockerfile --build-arg UID=$(UID) --build-arg GID=$(UID) --platform linux/amd64 .
	docker tag  $(TAGNAME) $(IMAGE)
rebuild: verify_var
	docker build --no-cache -t $(tagname) -f Dockerfile  --build-arg UID=$(UID) --build-arg GID=$(UID) --platform linux/amd64 .
	docker tag  $(TAGNAME) $(IMAGE)
no-docker-build:
	cargo b
deploy: verify_var
	@echo
	@echo "To start the service on DSH from the command line, you need to have the Rest Credentials."
	@echo "See https://console.dsh-dev.dsh.np.aws.kpn.com/#/profiles/$(strip $(TENANT))/rest-api"
	@echo
	@echo "Else start the service from console."
	@echo 
	@read -r -p "Do you want to start the service on DSH from command line? [y/N] " DEPLOY_CHOICE; \
	if [ "$$DEPLOY_CHOICE" = "y" ]; then \
		clear; \
		make exec_deploy; \
	else \
        echo "Deployment canceled."; \
    fi
exec_deploy: 
	@clear
	@bash -c 'bash deployer.sh "$(IMAGE)" "$(PLATFORM)" "$(strip $(TENANT))" "$(strip $(UID))" "$(TOPIC)" "$(CLIENT_ID)" "$(CLIENT_SECRET)"'
all: verify_var
	make build
	make push
	make show
	make deploy
run:
	docker run -u $(UID):$(UID) -it --entrypoint "/bin/sh" $(IMAGE)
fun:
	@curl -s -H "Accept: application/json" https://icanhazdadjoke.com/ 
push:
	@echo "Pushing image to Harbor: $(IMAGE)"
	docker push $(IMAGE)
show:
	@echo "#make file configuration"
	@echo "#URL          :" $(DOCKER_REPO_URL)
	@echo "#TENANT       :" $(TENANT)
	@echo "#TENANT_UID   :" $(UID)
	@echo "#TAG          :" $(TAGNAME)
	@echo "#version      :" $(VERSION)
	@echo "#image        :" $(IMAGE)
