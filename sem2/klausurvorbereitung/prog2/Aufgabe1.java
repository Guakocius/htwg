
public class Aufgabe1 {
  public static class ListNode {
    public ListNode next;
    public DataNode dataList;

    public ListNode(ListNode p) {
      this.next = p;
      this.dataList = null;
    }
  }

  public static class DataNode {
    public DataNode next;
    public int data;

    public DataNode(DataNode p, int x) {
      this.next = p;
      this.data = x;
    }
  }

  public static void main(String[] args) {
    ListNode lst = new ListNode(null);
    lst = new ListNode(lst);
    lst = new ListNode(lst);

    DataNode d1 = new DataNode(null, 5);
    DataNode d2 = new DataNode(null, 7);
    d2 = new DataNode(d2, 3);
    lst.dataList = d1;
    lst.next.next.dataList = d2;

    for (ListNode l = lst; l != null; l = l.next) {
      System.out.print("[");
      for (DataNode p = l.dataList; p != null; p = p.next) {
        System.out.print(p.data + ", ");
      }
      System.out.println("]");
    }
  }
}
