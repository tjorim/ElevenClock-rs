; Compile only through scripts/package-rust-release.ps1. This is a new identity;
; never reuse the legacy ElevenClock AppId or its uninstall/reset actions.
#ifndef PayloadDir
  #error PayloadDir is required
#endif
#ifndef PackageVersion
  #error PackageVersion is required
#endif
#ifndef ArtifactName
  #error ArtifactName is required
#endif
#ifndef ArtifactDir
  #error ArtifactDir is required
#endif

[Setup]
AppId={{97911B67-52D7-4D9B-B833-29303BC540A7}
AppName=ElevenClock-rs
AppVersion={#PackageVersion}
AppPublisher=ElevenClock-rs contributors
AppSupportURL=https://github.com/tjorim/ElevenClock-rs/issues
DefaultDirName={localappdata}\Programs\ElevenClock-rs
DisableDirPage=yes
DisableProgramGroupPage=yes
PrivilegesRequired=lowest
PrivilegesRequiredOverridesAllowed=
ArchitecturesAllowed=x64os
ArchitecturesInstallIn64BitMode=x64os
MinVersion=10.0.22000
OutputDir={#ArtifactDir}
OutputBaseFilename={#ArtifactName}-setup
Compression=lzma2
SolidCompression=yes
WizardStyle=modern
UninstallDisplayIcon={app}\elevenclock-rs.exe
LicenseFile={#PayloadDir}\LICENSE.txt
InfoBeforeFile={#PayloadDir}\README.txt
CloseApplications=no
RestartApplications=no
SetupMutex=ElevenClock-rs.Setup
SignedUninstaller=no

[Files]
; An explicit allowlist: no probes, scripts, PDBs, Python or Qt.
Source: "{#PayloadDir}\elevenclock-rs.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PayloadDir}\LICENSE.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PayloadDir}\THIRD-PARTY-NOTICES.txt"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PayloadDir}\RUST-COPYRIGHT.html"; DestDir: "{app}"; Flags: ignoreversion
Source: "{#PayloadDir}\README.txt"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{userprograms}\ElevenClock-rs"; Filename: "{app}\elevenclock-rs.exe"; WorkingDir: "{app}"; AppUserModelID: "tjorim.ElevenClockRs"

[Run]
Filename: "{app}\elevenclock-rs.exe"; Description: "Open ElevenClock-rs Settings"; Flags: nowait postinstall skipifsilent unchecked

[Code]
const
  RunKey = 'Software\Microsoft\Windows\CurrentVersion\Run';
  ApprovedKey = 'Software\Microsoft\Windows\CurrentVersion\Explorer\StartupApproved\Run';
  StartupValue = 'ElevenClockRust';
  ManagedLauncher = 'elevenclock-rs-launcher.exe';

function ManagedCopyMessage(): String;
begin
  Result := 'A copy managed by your organization is installed for this account. ' +
    'Setup will not change it.';
end;

function AppIsClosed(): Boolean;
begin
  Result := FindWindowByClassName('ElevenClock-rs.Runtime.Host') = 0;
  if not Result then
    SuppressibleMsgBox('Exit ElevenClock-rs from Settings or the tray menu, then try again.', mbInformation, MB_OK, IDOK);
end;

// The managed launcher (ADR 0004) deploys its copy into this same folder. A copy
// managed by the organization is never overwritten or uninstalled from here.
function ManagedCopyIn(Folder: String): Boolean;
begin
  Result := FileExists(AddBackslash(Folder) + ManagedLauncher);
end;

function InitializeSetup(): Boolean;
begin
  Result := not ManagedCopyIn(ExpandConstant('{localappdata}\Programs\ElevenClock-rs'));
  if not Result then
    SuppressibleMsgBox(ManagedCopyMessage(), mbError, MB_OK, IDOK)
  else
    Result := AppIsClosed();
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  Result := '';
  // Also covers an installation folder chosen with /DIR.
  if ManagedCopyIn(ExpandConstant('{app}')) then
    Result := ManagedCopyMessage()
  else if not AppIsClosed() then
    Result := 'ElevenClock-rs is still running. Exit it before continuing.';
end;

function InitializeUninstall(): Boolean;
begin
  Result := AppIsClosed();
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Command, OwnedCommand: String;
begin
  if CurUninstallStep = usUninstall then begin
    OwnedCommand := '"' + ExpandConstant('{app}\elevenclock-rs.exe') + '" --startup';
    if RegQueryStringValue(HKCU, RunKey, StartupValue, Command) then
      if CompareText(Trim(Command), OwnedCommand) = 0 then begin
        RegDeleteValue(HKCU, RunKey, StartupValue);
        RegDeleteValue(HKCU, ApprovedKey, StartupValue);
      end;
  end;
  // No wildcard deletion, no preferences/log removal, no legacy actions.
end;
