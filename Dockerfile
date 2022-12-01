# escape=`

FROM nicballesteros/rust-skia:latest

# Run the VS Dev Cmd Shell Helper
SHELL ["cmd", "/S", "/C", "C:\\shell.bat"]

WORKDIR C:\\Project

COPY . .

RUN cargo build --features windows --example text