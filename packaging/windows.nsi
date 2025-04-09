!include "MUI2.nsh"

Name "${APP_NAME}"
OutFile "${OUTPUT_FILE}"
InstallDir "$PROGRAMFILES\${APP_NAME}"

!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES

Section
  SetOutPath $INSTDIR
  File "${INPUT_DIR}\${APP_NAME}.exe"
  File "${INPUT_DIR}\icon.ico"
  CreateShortCut "$SMPROGRAMS\${APP_NAME}.lnk" "$INSTDIR\${APP_NAME}.exe"
SectionEnd
