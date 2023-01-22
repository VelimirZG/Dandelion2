FROM postgres:14.1
COPY . /docker-entrypoint-initdb.d
#COPY ./backup/ /backup/
#RUN apt-get update && apt-get -y install cron
#WORKDIR /backup
#RUN crontab crontab
#RUN touch cron_logs.txt
#RUN chmod +x backup.sh
#RUN chmod +x restore.sh
#RUN chmod +x cron_logs.txt
#RUN mkdir /backup/files
#RUN service cron start
