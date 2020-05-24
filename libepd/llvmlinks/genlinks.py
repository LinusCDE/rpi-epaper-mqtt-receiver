#!/usr/bin/env python3

import os

for file in os.listdir('/usr/bin/'):
  if not file.endswith('-7'):
    continue
  os.system('ln -s /usr/bin/%s %s' % (file, file[:-2]))
