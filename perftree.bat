@echo off
setlocal

set "depth=%~1"
set "fen=%~2"
set "moves=%~3"
set "engine=.\target\release\rusty-chess-bot.exe"

if "%depth%"=="" (
    echo Usage: %~nx0 ^<depth^> "fen" "[moves]"
    exit /b 1
)

(
    if not "%moves%"=="" (
        echo position fen %fen% moves %moves%
    ) else (
        echo position fen %fen%
    )
    echo go perft %depth% --perftree
    echo quit
) | "%engine%"

endlocal