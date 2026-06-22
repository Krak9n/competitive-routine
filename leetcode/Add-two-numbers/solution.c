#include <stdio.h>
#include <stdlib.h>

struct ListNode* addTwoNumbers(struct ListNode* l1, struct ListNode* l2) {
    struct ListNode *dummy = malloc(sizeof(struct ListNode));
    struct ListNode *current = dummy;
    int carry = 0;

    // i hate myself
    while (l1 || l2 || carry) {
        int v1 = (l1) ? l1->val : 0;
        int v2 = (l2) ? l2->val : 0;
        int sum = v1 + v2 + carry;

        carry = sum / 10;

        // creates new node
        current->next = malloc(sizeof(struct ListNode));
        current = current->next;
        current->val = sum % 10;
        current->next = NULL;

        if (l1) l1 = l1->next;
        if (l2) l2 = l2->next;
    }

    struct ListNode *result = dummy->next;
    free(dummy);
    return result;
}

int getLength(struct ListNode* head) {
    int count = 0;
    while (head) {
        count++;
        head = head->next;
    }
    return count;
}

void printList(struct ListNode* head) {
    while (head) {
        printf("%d", head->val);
        if (head->next) printf(" -> ");
        head = head->next;
    }
    printf("\n");
}
