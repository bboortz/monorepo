/*  
 *  hello.c
 */
#include <linux/module.h>	/* Needed by all modules */
#include <linux/kernel.h>	/* Needed for KERN_INFO */


MODULE_LICENSE("GPL");
MODULE_AUTHOR("Bejamin Boortz <benjamin.boortz@mailbox.org");
MODULE_DESCRIPTION("hello world");


int init_module(void) {
	printk(KERN_INFO "Hello World!!\n");
	return 0;
}


void cleanup_module(void) {
	printk(KERN_INFO "ByeBye World!\n");
}

