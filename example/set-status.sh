dbus-send \
  --session \
  --type="method_call" \
  --dest=fi.smuli.timetracker \
  --print-reply \
  /fi/smuli/timetracker/status \
  fi.smuli.timetracker.status.set string:"$1"
