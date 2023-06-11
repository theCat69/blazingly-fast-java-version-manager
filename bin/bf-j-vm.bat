rem Copyright (C) 2023 Félix Vadcard
rem see LINCENCE.txt for details

@echo off 

SET ARGS_LIST=%*

set "switch_long=switch"
set "switch_short=s"
set "switch=false"

for %%a in (%ARGS_LIST%) do (
  rem we look for local because on windows it is not possible to change env variable from current prompt  
  if "%%a"=="%switch_long%" (
    set "switch=true"
  ) else if "%%a"=="%switch_short%" (
    set "switch=true"
  )
)

:EndLoop

if "%switch%"=="true" (
  goto :Switch
)

%~dp0\deps\bf-j-vm.exe %* 

goto :End

:Switch 

set "temp_dir=%~dp0tmp_work"
set "temp_file=%temp_dir%\output.txt"
  
if not exist %temp_dir% mkdir %temp_dir%

rem this will write to a %temp_file% so we can read from it after execution
%~dp0\deps\bf-j-vm.exe %* 

IF %ERRORLEVEL% NEQ 0 (
  echo Error from bf-j-vm.exe look above for details
  goto :EndSwitch
)

for /F "tokens=1,2 delims=|" %%i in ('type %temp_file%') do (
  set "arg1=%%i"
  set "arg2=%%j"
)

set JAVA_HOME=%arg1% 
set PATH=%arg2% 

:EndSwitch

del /s %temp_file% >nul 2>&1
rmdir %temp_dir%

goto :End

:End
