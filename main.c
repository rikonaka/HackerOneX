#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <curl/curl.h>

#include "cJSON.h"

#define MAX_LINE 12800

CURLcode res;
CURL *curl_handle;

size_t write_data(void *ptr, long unsigned int size, long unsigned int nmemb, void *stream)
{
    /*if (strlen((char *)stream) + strlen((char *)ptr) > MAX_LINE)
    {
        return 0;
    }*/
    strcat(stream, (char *)ptr);
    return size*nmemb;
}

char *down_file(char *url_address)
{
    static char str[MAX_LINE];
    char *p2s;
    strcpy(str, "");
    
    /* set the download address */
    curl_easy_setopt(curl_handle, CURLOPT_URL, url_address);
    /* set the timeout */
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 100);
    /* set the writedata function */
    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_data);
    /* set the wirte variable */
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, str);
    /* set the user-agent field */
    curl_easy_setopt(curl_handle, CURLOPT_USERAGENT, "libcurl-agent/1.0");
    
    /* download */
    res = curl_easy_perform(curl_handle);
    //str[MAX_LINE-1] = "\0";
    for (static int i = 0; i < strlen(str); i ++)
    {
        p2s = &str[i];
        if (*p2s != "\0")
    }
    if(res != CURLE_OK)
    {
        fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
        return NULL; //judge the download if successful
    }
    else
    {
        printf("%lu bytes retrieved\n", strlen(str));
    }
    return str;
}

int main(int argc, char *argv[])
{
    char url[CHAR_MAX];
    char *purl = url;
    char *result;
    char *one_result;
    char *one_json;
    
    /* init the curl */
    curl_handle = curl_easy_init();
    
    int i;
    for (i = 1; i < argc; i++)
    {
        one_result = argv[i];
        sprintf(purl, "http://www.exploitalert.com/api/search-exploit?name=%s", one_result);
        
        result = down_file(url);
        
        if (result)
        {
            /* deal with the JSON data */
            /* [{"id":"19952","date":"2014-10-10","name":"WordPress Google Calendar Events 2.0.1 Cross Site Scripting"}, ...] */
            cJSON *root = NULL;
            
            root = cJSON_Parse(result);
            if (root == NULL)
            {
                printf("Get root failed\n");
                exit(1);
            }
            cJSON *id = cJSON_GetObjectItem(root, "id");
            
            if (!id)
            {
                printf("Can't get id\n");
                exit(1);
            }
            cJSON *date = cJSON_GetObjectItem(root, "date");
            if (!date)
            {
                printf("Can't get date\n");
                exit(1);
            }
            cJSON *name = cJSON_GetObjectItem(root, "name");
            if (!name)
            {
                printf("Can't get name\n");
                exit(1);
            }
            printf("%s %s %s\n", id->valuestring, date->valuestring, name->valuestring);
            
            if (root)
            {
                cJSON_Delete(root);
            }
        }
        else
        {
            printf("NO %s vulnerability has been found, program exitting", one_result);
        }
    }
    curl_easy_cleanup(curl_handle); //release curl
    return 0;
}
