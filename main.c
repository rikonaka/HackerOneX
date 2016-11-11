#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <unistd.h>

#include "cJSON.h"
#include <curl/curl.h>

#define TMP_FILE "tmp.txt"
#define SPLIT_FILE "split.txt"

CURLcode res;
CURL *curl_handle;

int split_c(void)
{
    char read_buf[10];
    char read_buf_for_end[10];
    char comp_1[2] = {"["};
    char comp_2[2] = {"]"};
    char comp_3[2] = {"{"};
    char comp_4[2] = {"}"};
    
    int count_each_split = 0;
    
    FILE *tmp_fp;
    FILE *split_fp;
    
    tmp_fp = fopen(TMP_FILE, "r");
    split_fp = fopen(SPLIT_FILE, "a+");
    while ((read_buf[0] = fgetc(tmp_fp)) != EOF)
    {
        //printf("1\n");
        if ((read_buf[0] != comp_1[0]) && (read_buf[0] != comp_2[0]))
        {
            //printf("    2\n");
            if (read_buf[0] == comp_3[0])
            {
                //printf("        3\n");
                count_each_split = 1;
                fwrite(read_buf, sizeof(char), 1, split_fp);
                continue;
            }
            else if (read_buf[0] == comp_4[0])
            {
                //printf("            4\n");
                count_each_split = 0;
                sprintf(read_buf_for_end, "%s\n", read_buf);
                fwrite(read_buf_for_end, 2 * sizeof(char), 1, split_fp);
                continue;
            }
            else if (count_each_split)
            {
                //printf("                5\n");
                fwrite(read_buf, sizeof(char), 1, split_fp);
                continue;
            }
        }
    }
    fclose(tmp_fp);
    fclose(split_fp);
    return 0;
}

size_t write_data(void *ptr, size_t size, size_t nmemb, void *stream)
{
    size_t data_size = size * nmemb;
    static FILE *fp = NULL;
    if (access((char *)stream, 0) == -1)
    {
        fp = fopen((char *)stream, "wb");
    }
    else
    {
        fp = fopen((char *)stream, "ab");
    }
    if (fp)
    {
        fwrite(ptr, size, nmemb, fp);
    }
    fclose(fp);
    return data_size;
}

char *down_file(char *url_address)
{
    static int down_size;
    /* set the download address */
    curl_easy_setopt(curl_handle, CURLOPT_URL, url_address);
    /* set the timeout */
    curl_easy_setopt(curl_handle, CURLOPT_TIMEOUT, 100);
    /* set the writedata function */
    curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, TMP_FILE);
    /* set the user-agent field */
    curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, write_data);
    /* set the wirte variable */
    curl_easy_setopt(curl_handle, CURLOPT_USERAGENT, "SVF-libcurl-agent/1.1");
    
    /* download */
    res = curl_easy_perform(curl_handle);
    //str[MAX_LINE-1] = "\0";
    if(res != CURLE_OK)
    {
        fprintf(stderr, "curl_easy_perform() failed, error message is: %s\n", curl_easy_strerror(res));
        return NULL; //judge the download if successful
    }
    else
    {
        printf("Get the API data retrieved\n");
    }
    return 0;
}

int libcurl_c(char *string)
{
    char url[100];
    char *purl = url;
    char *r;
    
    /* init the curl */
    curl_handle = curl_easy_init();
    
    int i;
    
    sprintf(purl, "http://www.exploitalert.com/api/search-exploit?name=%s", string);
    
    r = down_file(url);
    
    if (r != 0)
    {
        printf("libcurl module can't do it job");
        return -1;
    }
        
        
    return 0;
}

int main(int argc, char *argv[])
{
    FILE *splitfp;
    
    char everyline[128];
    
    cJSON *id;
    cJSON *date;
    cJSON *name;
    cJSON *root;
    
    int libresult = libcurl_c(argv[1]);
    if (libresult == 0)
    {
        int access_file = access(TMP_FILE, 0);
        if (access_file != 0)
        {
            printf("Can not found the tmp file");
            return 1;
        }
        else
        {
            int splitres = split_c();
            if (splitres == 0)
            {
                printf("NOW LIST THE ALL RESULT:\n");
                printf("NUML NUMC | DATE       | DETAIL\n");
                splitfp = fopen(SPLIT_FILE, "r");
                int count = 1;
                while (fgets(everyline, 1024, splitfp) != NULL)
                {
                    root = cJSON_Parse(everyline);
                    if (root != NULL)
                    {
                        id = cJSON_GetObjectItem(root, "id");
                        date = cJSON_GetObjectItem(root, "date");
                        name = cJSON_GetObjectItem(root, "name");
                        printf(">[%d] %s | %s | %s\n", count, id-> valuestring, date->valuestring, name->valuestring);
                        cJSON_Delete(root);
                        count ++;
                    }
                }
                fclose(splitfp);
            }
        }
    }
    if (remove(TMP_FILE) == 0)
    {
        if (remove(SPLIT_FILE) == 0)
        {
            printf("END");
        }
    }
    return 0;
}
