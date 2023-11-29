TENANT=greenbox-dev
TENANTUID=1903

DOCKER_REPO_URL=registry.cp.kpn-dsh.com/$(TENANT)
VERSION=example
tagname=dsh-sdk-example


image=$(DOCKER_REPO_URL)/$(tagname):$(VERSION)

help:
	@echo "login		- login to the relevant repository"
	@echo "fix		- run dos2unix on every file"
	@echo "build		- build the image"
	@echo "rebuild		- build the image with the --no-cache flag"
	@echo "no-docker-build	- Cargo build on local machine"
	@echo "all		- shorthand for fix, build, push, show"
	@echo "run		- run the image in local docker"
	@echo "push		- push the  image to jfrog"
	@echo "show		- show the current make variables"
login:
	docker login $(DOCKER_REPO_URL) 
fix: 
	find . -type f -print0 | xargs -0 dos2unix
build:
	docker build -t $(tagname) -f Dockerfile --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(tagname) $(image)
rebuild:
	docker build --no-cache -t $(tagname) -f Dockerfile  --build-arg UID=$(TENANTUID) --build-arg GID=$(TENANTUID) --platform linux/amd64 .
	docker tag  $(tagname) $(image)
no-docker-build:
	cargo b
all:
	make build
	make push
	make show
run:
	docker run -u $(TENANTUID):$(TENANTUID) -it --entrypoint "/bin/sh" $(image)
fun:
	@curl -s -H "Accept: application/json" https://icanhazdadjoke.com/ 
push:
	docker push $(image)
show:
	@echo "#make file configuration"
	@echo "#URL          :" $(DOCKER_REPO_URL)
	@echo "#TENANT       :" $(TENANT)
	@echo "#TENANTUID 	 :" $(TENANTUID)
	@echo "#TAG          :" $(tagname)
	@echo "#version      :" $(VERSION)
	@echo "#image        :" $(image)
