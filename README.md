# tok-dl

## Description

tok-dl is a command-line program that, with the help of yt-dlp, partially automates the mass-downloading of TikTok videos. It is a quick and simple program I wrote to help me download all videos of all TikTok users I follow, since all the existing tools I could find that could supposedly download TikTok videos en masse (youtube-dl, yt-dlp, tiktok-dl, tiktok-scraper, and the myfaveTT Chrome extension) do not work as of January 2nd, 2023.

This program does not fully automate the process and may still require many hours of manual labor, depending on how many videos are to be downloaded and how many users those videos are spread across. However, it is still orders of magnitude faster than downloading videos one at a time.

## Usage
```
tok-dl.exe <COMMAND>

Commands:
  user        Use the provided web page to download all videos from the user that the page belongs
              to
  batch-user  Use the web pages in the provided directory to download all videos from the users
              that the pages belong to
  list-file   Download all videos listed in the provided file
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help information
  -V, --version  Print version information



tok-dl.exe user --html-page <HTML_PAGE> --download-dir <DOWNLOAD_DIR>

Options:
  -i, --html-page <HTML_PAGE>        Path to user's HTML page
  -d, --download-dir <DOWNLOAD_DIR>  Path of the directory to download the videos to
  -h, --help                         Print help 



tok-dl.exe batch-user --html-dir <HTML_DIR> --download-dir <DOWNLOAD_DIR>

Options:
  -i, --html-dir <HTML_DIR>          Path to directory containing all users' HTML pages       
  -d, --download-dir <DOWNLOAD_DIR>  Path of the directory to download the videos to
  -h, --help                         Print help information



tok-dl.exe list-file --list-file <LIST_FILE> --download-dir <DOWNLOAD_DIR>

Options:
  -i, --list-file <LIST_FILE>        Path to plain-text file containing line-separated TikTok video
                                     URLs
  -d, --download-dir <DOWNLOAD_DIR>  Path of the directory to download the videos to
  -h, --help                         Print help information
```

## Guide

### Download all videos uploaded by a single TikTok user
1. Follow the steps in the "Download TikTok user's video page" section.
2. Use the `user` command of tok-dl, entering the path of the HTML file of the web page you just downloaded and the path of the directory you want the downloaded videos to be placed in. e.g. `tok-dl user --html-page "html_pages/The Rock (@therock) Official TikTok.htm" --download-dir downloads/therock`

### Download all videos uploaded by multiple TikTok users
1. Follow the steps in the "Download TikTok user's video page" section for every user whose videos you want to download, and place all the web pages in a single directory.
2. Use the `batch-user` command of tok-dl, entering the path of the directory containing the HTML files and the path of the directory you want the downloaded videos to be placed in (a subdirectory will be created for each user). e.g. `tok-dl batch-user --html-dir html_pages --download-dir downloads`

### Download all videos listed in a plain text file
1. Create a plain text file (i.e. a `txt` file) and enter one TikTok video URL per line.
2. Use the `list-file` command of tok-dl, entering the path of the plain text file containing the list of URLs of videos to download and the path of the directory you want the downloaded videos to be placed in (a subdirectory will be created for each user). e.g. `tok-dl batch-user --html-dir html_pages --download-dir downloads`

### Download TikTok user's video page
1. Visit the TikTok user's page through tiktok.com (the website, not the app) on a desktop/laptop. e.g. https://www.tiktok.com/@therock
2. Scroll down until all the user's videos are loaded. This can be done more quickly and automatically by using the autoscroll feature of the web browser--middle-click (just click; do not hold) at the top of the page, then move the mouse cursor to the very bottom of the screen, and wait for all of the videos to finish loading. This can take a long time if the user has a lot of videos, and sometimes no more videos will load even when not all videos have actually loaded. Unfortunately, the only way to ensure you have actually loaded all the videos is to reload the page, repeat the process, and check to see if the last video is the same both times.
3. Save the complete web page with your web browser using `Ctrl + S`, or by right-clicking and selecting `Save Page As...` on Firefox, or `Save as...` on Chrome. To ensure that you are downloading the _complete_ web page, not just the HTML file, select the `Web Page, complete` option on Firefox, or the `Webpage, Complete` option on Chrome when selecting the directory to save the web page. Sometimes the page will fail to download, in which case you should manually delete both the downloaded folder and HTML page and try again. If you have access to a VPN, then using that to change your IP address (by switching to a different server, for example) can sometimes remedy the problem.
