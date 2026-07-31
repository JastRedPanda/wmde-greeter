#!/bin/sh
rm -rf /run/wmde-greeter/wmde/fun.wmde.SettingsDaemon/v1/* > /dev/null 2>&1
exec wmde-comp wmde-greeter > /dev/null 2>&1