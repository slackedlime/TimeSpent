; Script generated by the Inno Setup Script Wizard and Modified by me.
; SEE THE DOCUMENTATION FOR DETAILS ON CREATING INNO SETUP SCRIPT FILES!

#define MyAppName "TimeSpent"
#define MyAppVersion "1.2"
#define MyAppPublisher "Slacked Lime"
#define MyAppURL "https://github.com/slackedlime/TimeSpent"
#define MyAppExeName "TimeSpent.exe"

[Setup]
; NOTE: The value of AppId uniquely identifies this application. Do not use the same AppId value in installers for other applications.
; (To generate a new GUID, click Tools | Generate GUID inside the IDE.)
AppId={{00659E63-BD87-4602-815D-71D2BB6419FA}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppVerName=v{#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}
DefaultDirName={autopf}\{#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=LICENSE
; Remove the following line to run in administrative install mode (install for all users.)
PrivilegesRequired=lowest
OutputDir=target
OutputBaseFilename=TimeSpent Setup v{#MyAppVersion}
SetupIconFile=imgs\hummingbird_new.ico
Compression=lzma
SolidCompression=yes
WizardStyle=classic
WizardSizePercent=120
DisableWelcomePage=no
WizardImageFile=imgs\hummingbird_new.bmp

[Messages]
WelcomeLabel2=This will install [name] on your computer. \
%n%nA simple GUI rust application that keeps track of how much time you spend on each application. \
%n%nThis application is Open Source, and you can contribute to it on Github.

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Files]
Source: "target\release\gui.exe"; DestName: "{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion
Source: "target\release\daemon.exe"; DestName: "TimeSpentDaemon.exe"; DestDir: "{app}"; Flags: ignoreversion
; NOTE: Don't use "Flags: ignoreversion" on any shared system files

[Icons]
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Tasks: desktopicon
Name: "{userstartup}\TimeSpentDaemon"; Filename: "{app}\TimeSpentDaemon.exe"

[Run]
Filename: "{app}\TimeSpentDaemon.exe"; Description: "Launch Daemon"; Flags: nowait runasoriginaluser
Filename: "{app}\{#MyAppExeName}"; Description: "{cm:LaunchProgram,{#MyAppName}}"; Flags: nowait postinstall skipifsilent

[UninstallRun]
Filename: "taskkill.exe"; Parameters: "/IM TimeSpentDaemon.exe /F"; RunOnceId: "KillDaemon"; Flags: runhidden

[code]
function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  ResultCode: integer;
begin
  // Kill TimeSpentDaemon if it is running
  Exec('taskkill.exe', '/IM TimeSpentDaemon.exe /F', '', SW_HIDE, ewWaitUntilTerminated, ResultCode);
  
  Result := '';
end;
