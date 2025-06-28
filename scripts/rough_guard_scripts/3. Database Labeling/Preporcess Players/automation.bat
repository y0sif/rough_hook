@echo off

cd F:\Linux\rough_guard_db\scripts\player_filter

:loop
python F:\Linux\rough_guard_db\scripts\player_filter\player_labeling.py F:\Linux\rough_guard_db\scripts\player_filter\all_players.json
if %errorlevel% neq 1 (
    echo [ERROR] Loop Broken: %errorlevel%
    pause
    exit /b
)

net stop CloudflareWARP
timeout /t 60

net start CloudflareWARP
timeout /t 60

goto loop
