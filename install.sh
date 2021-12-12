#!/bin/sh
red="\033[1;31m"
green="\e[0;92m"
yellow="\033[1;33m"
blue="\033[1;34m"
purple="\033[1;35m"
cyan="\033[1;36m"
grey="\033[0;37m"
bold="\e[1m"
uline="\e[4m"
reset="\e[0m"

echo "${blue}${bold}=======================${reset}"
echo "${blue}${bold}CodeOpen Install Script${reset}"
echo "${blue}${bold}=======================${reset}\n"
echo "${green}${bold}This process is going to ask for your sudo password, just so we can move the file to /usr/local/bin${reset}\n"

if [[ $(uname) == "Linux" ]]
then
        # Linux
        echo -e "${blue}${bold}=======================${reset}"
        echo -e "${blue}${bold}CodeOpen Install Script${reset}"
        echo -e "${blue}${bold}=======================${reset}\n"
        echo -e "${green}${bold}This process is going to ask for your sudo password, just so we can move the file to /usr/local/bin${reset}\n"


        echo -e "${white}${bold}Downloading Tar File...${reset}"
        curl --progress-bar -L https://github.com/ShubhamPatilsd/codeopen/releases/download/release/codeopen_release_x86_64-unknown-linux-musl.tar.xz --output codeopen_release_x86_64-unknown-linux-musl.tar.xz
        tar -xf codeopen_release_x86_64-unknown-linux-musl.tar.xz
        echo -en "${red}Move to \`/usr/local/bin\`? (y/n):${reset} "
        read answer
        if [ "$answer" != "${answer#[Yy]}" ] ;then 
            echo -e "\n${green}${bold}Moving binary...${reset}"
            sudo mv codeopen /usr/local/bin/
            sleep 1
        fi
        rm -r codeopen_release_x86_64-unknown-linux-musl.tar.xz
        echo -e "\n\n${yellow}${bold}That's it! You're up and running with Codeopen. Next, read the docs at https://github.com/ShubhamPatilsd/codeopen/blob/main/docs/guide.md to learn how to use it!${reset}\n"
elif [[ $(uname) == "Darwin" ]]
then
        # macOS
        echo "${blue}${bold}=======================${reset}"
        echo "${blue}${bold}CodeOpen Install Script${reset}"
        echo "${blue}${bold}=======================${reset}\n"
        echo "${green}${bold}This process is going to ask for your sudo password, just so we can move the file to /usr/local/bin${reset}\n"
        echo "${white}${bold}Downloading Zip File..."
        curl --progress-bar -L https://github.com/shubhamPatilsd/codeopen/releases/latest/download/codeopen_release_x86_64-apple-darwin.zip --output codeopen_release_x86_64-apple-darwin.zip
        unzip codeopen_release_x86_64-apple-darwin.zip
        echo -n "${red}Move to \`/usr/local/bin\`? (y/n):${reset} "
        read answer
        if [ "$answer" != "${answer#[Yy]}" ] ;then 
            echo "\n${green}${bold}Moving binary...${reset}"
            sudo mv codeopen /usr/local/bin/
            sleep 1
        fi
        rm -r codeopen_release_x86_64-apple-darwin.zip
        echo "\n\n${yellow}${bold}That's it! You're up and running with Codeopen. Next, read the docs at https://github.com/ShubhamPatilsd/codeopen/blob/main/docs/guide.md to learn how to use it!${reset}\n"

fi

