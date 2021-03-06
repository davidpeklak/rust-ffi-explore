#include <stddef.h>
#include "ffiexp.h"
#include <errno.h>
#include <sys/event.h>

int mysocket () {
  return socket(AF_INET, SOCK_STREAM, 0);
}

int clientconnect(int sockfd, struct hostent *server, int portno) {
  struct sockaddr_in serv_addr;

  memset((char *) &serv_addr, 0, sizeof(serv_addr));
  serv_addr.sin_family = AF_INET;
  bcopy((char *)server->h_addr,
        (char *)&serv_addr.sin_addr.s_addr,
        server->h_length);
  serv_addr.sin_port = htons(portno);
  return connect(sockfd,(struct sockaddr *) &serv_addr,sizeof(serv_addr));
}

int serverconnect(int sockfd, int portno) {
  struct sockaddr_in serv_addr;

  memset((char *) &serv_addr, 0, sizeof(serv_addr));
  serv_addr.sin_family = AF_INET;
  serv_addr.sin_addr.s_addr = INADDR_ANY;
  serv_addr.sin_port = htons(portno);
  return bind(sockfd, (struct sockaddr *) &serv_addr, sizeof(serv_addr));
}

int mywrite(int sockfd, char *buf) {
  return write(sockfd, buf, strlen(buf));
}

int my_errno() {
  return errno;
}

void *allocBuf(int len) {
  void* buf = malloc(len + 1);
  memset(buf, 0, len + 1);
  return buf;
}

void printP(void *ptr) {
  printf("Ptr: %p\n", ptr);
}

void freeBuf(void *buf) {
  char *cbuf = (char *) buf;
  if (cbuf != NULL) free(cbuf);
}

void *id(void *ptr) {
  return ptr;
}

int myaccept(int sockfd) {
  return accept(sockfd, NULL, NULL);
}

void my_ev_set(struct kevent *ev, int sockfd, void *tag) {
  EV_SET(ev, sockfd, EVFILT_READ, EV_ADD, 0, 0, tag);
}

int keventfn(int kq, const struct kevent *changelist, int nchanges,
  struct kevent *eventlist, int nevents) {
  return kevent(kq, changelist, nchanges, eventlist, nevents, NULL);
}