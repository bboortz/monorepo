+++
title = "How IPFS is showing up-to-date Information"
date = "2021-09-23T19:31:40+02:00"
author = ""
authorTwitter = "" #do not include @
cover = ""
tags = []
keywords = ["", ""]
description = ""
showFullContent = true
+++

After a few small changes of the last article I have seen that the current IPFS has been changed to a newer one. 
Changing is not the acuarate working. A new hash been created and the old one exists too but will not be used anymore.
The current one is `Qmd1vjCbc5yzndS6TZg9KS5hhPVNyJL2RX8DpxWGD2Jeop` which is already outdated after posting this article.
How do I have observed this? 

1. First the hash that [Fleek](https://fleek.co) is pointing to has been changed on the UI.
2. Secondary `dig +short TXT _dnslink.benni.tech | sed -E 's/"dnslink=(.*)"/\1/g'` is pointing now to `/ipfs/Qmd1vjCbc5yzndS6TZg9KS5hhPVNyJL2RX8DpxWGD2Jeop`
2. Third [https://ipfs.io/ipfs/QmdxaxczCgnNgDA4wy6gjAJoKqXFKJiYhquKYqUHMk4ysu/](https://ipfs.io/ipfs/QmdxaxczCgnNgDA4wy6gjAJoKqXFKJiYhquKYqUHMk4ysu/) was still showing the old data.
Secondary
4. Fourth `ipfs dns benni.tech` is pointing to `/ipfs/Qmd1vjCbc5yzndS6TZg9KS5hhPVNyJL2RX8DpxWGD2Jeop` too
5. Fifth `ipfs cat /ipfs/Qmd1vjCbc5yzndS6TZg9KS5hhPVNyJL2RX8DpxWGD2Jeop/index.html` is showing up-to-date data.

So that 
1. for all new data will be a new IPFS hash created
2. [Fleek](https://fleek.co) is pointing after every update to the current/latest IPFS hash. This is how [Fleek](https://fleek.co) is ensuring the latest page will shown.

