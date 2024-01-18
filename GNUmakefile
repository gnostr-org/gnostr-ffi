PROJECTS=\
c-to-rust\
cpp-to-rust\
crystal-to-rust\
csharp-to-rust\
dart-to-rust\
go-to-rust-cgo-dynamic\
go-to-rust-cgo-static\
haskell-to-rust\
julia-to-rust\
luajit-to-rust\
node-to-rust\
perl-to-rust\
php-to-rust\
python-to-rust\
ruby-to-rust\
rust-to-c\
rust-to-cmake\
rust-to-cpp\
rust-to-go-dynamic\
swift-to-rust
export PROJECTS

FORTRAN=\
rust-to-fortran

-:
## 	@( \
##     LIST=$$(find * -maxdepth 0 -type d) && for e in $$LIST; do echo "$$e"; done;\
##     );
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

all:$(PROJECTS)## 	all
.PHONY:$(PROJECTS)
$(PROJECTS):##
	cd $@ && cargo b
	cd $@ && cargo t
rust-to-fortran:## 	rust-to-fortran
	cd rust-to-fortran && cargo b && cargo t
