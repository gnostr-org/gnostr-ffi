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
rust-to-c\
##rust-to-cmake\
##rust-to-cpp\
##rust-to-go-dynamic\
swift-to-rust
export PROJECTS

FORTRAN=\
rust-to-fortran
RUBY=\
ruby-to-rust

default:
## 	@( \
##     LIST=$$(find * -maxdepth 0 -type d) && for e in $$LIST; do echo "$$e"; done;\
##     );
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

.PHONY:$(PROJECTS) gnostr rust-to-fortran ruby-to-rust
all:$(PROJECTS) gnostr## 	all
gnostr:## 	gnostr
	@cd gnostr && make
$(PROJECTS):##
	cd $@ && cargo b && make || true
	cd $@ && cargo t
rust-to-fortran:## 	rust-to-fortran
	cd rust-to-fortran && cargo b && cargo t
ruby-to-rust:## 	ruby-to-rust
	cd ruby-to-rust && cargo b && cargo t

-include Makefile
