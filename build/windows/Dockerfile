# escape=`

# Use the latest Windows Server Core 2019 image.
FROM mcr.microsoft.com/windows/servercore:ltsc2019 AS build

# Use powershell commands
SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

# Install Chocolatey
RUN Set-ExecutionPolicy Bypass -Scope Process -Force; `
    [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; `
    iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install Build Tools for Clang
RUN Write-Host ('Installing Dependencies'); `
    choco install python -y; `
    choco install gnuwin -y; `
    choco install cmake -y --installargs 'ADD_CMAKE_TO_PATH=System'; `
    choco install git -y

# Update GnuWin32 to Path
RUN $env:PATH = $env:PATH + ';c:\GnuWin\bin'; `
    Set-ItemProperty -Path 'HKLM:\SYSTEM\CurrentControlSet\Control\Session Manager\Environment' -Name Path -Value $env:PATH

# Verify tools were installed correctly
RUN Write-Host ('Verifying Install'); `
    python --version; `
    cmake --version; `
    git --version

# Clone llvm
RUN git clone https://github.com/llvm/llvm-project.git

RUN choco install ninja -y

SHELL ["cmd", "/S", "/C"]

# Install VS Build Tools
RUN `
    # Download the Build Tools bootstrapper.
    curl -SL --output vs_buildtools.exe https://aka.ms/vs/17/release/vs_buildtools.exe `
    `
    # Install Build Tools with the Microsoft.VisualStudio.Workload.AzureBuildTools workload, excluding workloads and components with known issues.
    && (start /w vs_buildtools.exe --quiet --wait --norestart --nocache `
        --installPath "%ProgramFiles(x86)%\Microsoft Visual Studio\2022\BuildTools" `
        --add Microsoft.VisualStudio.Workload.VCTools `
        --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64 `
        --add Microsoft.VisualStudio.Workload.UniversalBuildTools `
        --add Microsoft.VisualStudio.Component.Windows10SDK.19041 `
        --add Microsoft.VisualStudio.Component.VC.ATL `
        --remove Microsoft.VisualStudio.Component.Windows10SDK.10240 `
        --remove Microsoft.VisualStudio.Component.Windows10SDK.10586 `
        --remove Microsoft.VisualStudio.Component.Windows10SDK.14393 `
        --remove Microsoft.VisualStudio.Component.Windows81SDK `
        || IF "%ERRORLEVEL%"=="3010" EXIT 0) `
    `
    # Cleanup
    && del /q vs_buildtools.exe

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

# Make build directory
RUN mkdir llvm-project/build

WORKDIR C:\llvm-project\build

# RUN Write-Host $env:PATH

# RUN 'C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat' -host_arch=amd64 -arch=amd64 && `
    # echo %PATH:;=&echo.%

# RUN Write-Host $env:PATH

ARG CC="cl"
ARG CXX="cl"

COPY shell.bat .

SHELL ["cmd", "/S", "/C", "shell.bat"]

# Build ninja files
RUN cmake -GNinja `
    -DCMAKE_BUILD_TYPE=Release `
    -DLLVM_ENABLE_PROJECTS=clang `
    -DLLVM_TARGETS_TO_BUILD='X86' `
    -DLLVM_BUILD_TOOLS=Off `
    -DLLVM_BUILD_RUNTIME=Off `
    -DLLVM_INCLUDE_TESTS=Off `
    -DLLVM_INCLUDE_EXAMPLES=Off `
    -DLLVM_ENABLE_BACKTRACES=Off `
    -DLLVM_ENABLE_OCAMLDOC=Off `
    -DLLVM_BUILD_UTILS=Off `
    -DLLVM_BUILD_DOCS=Off `
    -DLLVM_OPTIMIZED_TABLEGEN=On `
    -DCLANG_ENABLE_ARCMT=Off `
    -DCLANG_ENABLE_STATIC_ANALYZER=Off `
    -DCLANG_INCLUDE_TESTS=Off `
    -DCLANG_BUILD_EXAMPLES=Off `
    ..\llvm

# Build clang
RUN ninja clang

# Make new build stage clang
FROM mcr.microsoft.com/windows/servercore:ltsc2019 AS clang

# Use powershell as the shell
SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

# Save the location of VS as a build environment variable
ARG vs="C:\Program Files (x86)\Microsoft Visual Studio"

# Copy Visual Studio Build Tools 2022
COPY --from=build ${vs} ${vs}

# Copy the compiled binaries
COPY --from=build C:\llvm-project\build\bin C:\llvm\bin

# Add clang to PATH
RUN setx /M PATH $($Env:PATH + ';C:\llvm\bin')

# Copy Docker VS Dev Tools Shell Helper
COPY shell.bat .

# Define the entry point for the docker container.
# This entry point starts the developer command prompt and launches the PowerShell shell.
ENTRYPOINT ["C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\Common7\\Tools\\VsDevCmd.bat", "-host_arch=amd64", "-arch=amd64", "&&", "powershell.exe", "-NoLogo", "-ExecutionPolicy", "Bypass"]
# ENTRYPOINT ["powershell.exe"]