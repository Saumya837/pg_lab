#include "/Users/saumyakumar/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/pgrx-pg-sys-0.19.1/include/pg15.h"

// Static wrappers

float4 DatumGetFloat4__pgrx_cshim(Datum X) { return DatumGetFloat4(X); }
Datum Float4GetDatum__pgrx_cshim(float4 X) { return Float4GetDatum(X); }
float8 DatumGetFloat8__pgrx_cshim(Datum X) { return DatumGetFloat8(X); }
Datum Float8GetDatum__pgrx_cshim(float8 X) { return Float8GetDatum(X); }
ListCell * list_head__pgrx_cshim(const List *l) { return list_head(l); }
ListCell * list_tail__pgrx_cshim(const List *l) { return list_tail(l); }
ListCell * list_second_cell__pgrx_cshim(const List *l) { return list_second_cell(l); }
int list_length__pgrx_cshim(const List *l) { return list_length(l); }
ListCell * list_nth_cell__pgrx_cshim(const List *list, int n) { return list_nth_cell(list, n); }
ListCell * list_last_cell__pgrx_cshim(const List *list) { return list_last_cell(list); }
void * list_nth__pgrx_cshim(const List *list, int n) { return list_nth(list, n); }
int list_nth_int__pgrx_cshim(const List *list, int n) { return list_nth_int(list, n); }
Oid list_nth_oid__pgrx_cshim(const List *list, int n) { return list_nth_oid(list, n); }
int list_cell_number__pgrx_cshim(const List *l, const ListCell *c) { return list_cell_number(l, c); }
ListCell * lnext__pgrx_cshim(const List *l, const ListCell *c) { return lnext(l, c); }
ForEachState for_each_from_setup__pgrx_cshim(const List *lst, int N) { return for_each_from_setup(lst, N); }
ForEachState for_each_cell_setup__pgrx_cshim(const List *lst, const ListCell *initcell) { return for_each_cell_setup(lst, initcell); }
ForBothCellState for_both_cell_setup__pgrx_cshim(const List *list1, const ListCell *initcell1, const List *list2, const ListCell *initcell2) { return for_both_cell_setup(list1, initcell1, list2, initcell2); }
FullTransactionId FullTransactionIdFromEpochAndXid__pgrx_cshim(uint32 epoch, TransactionId xid) { return FullTransactionIdFromEpochAndXid(epoch, xid); }
FullTransactionId FullTransactionIdFromU64__pgrx_cshim(uint64 value) { return FullTransactionIdFromU64(value); }
void FullTransactionIdRetreat__pgrx_cshim(FullTransactionId *dest) { FullTransactionIdRetreat(dest); }
void FullTransactionIdAdvance__pgrx_cshim(FullTransactionId *dest) { FullTransactionIdAdvance(dest); }
TransactionId ReadNextTransactionId__pgrx_cshim(void) { return ReadNextTransactionId(); }
TransactionId TransactionIdRetreatedBy__pgrx_cshim(TransactionId xid, uint32 amount) { return TransactionIdRetreatedBy(xid, amount); }
TransactionId TransactionIdOlder__pgrx_cshim(TransactionId a, TransactionId b) { return TransactionIdOlder(a, b); }
TransactionId NormalTransactionIdOlder__pgrx_cshim(TransactionId a, TransactionId b) { return NormalTransactionIdOlder(a, b); }
FullTransactionId FullTransactionIdNewer__pgrx_cshim(FullTransactionId a, FullTransactionId b) { return FullTransactionIdNewer(a, b); }
Datum fastgetattr__pgrx_cshim(HeapTuple tup, int attnum, TupleDesc tupleDesc, bool *isnull) { return fastgetattr(tup, attnum, tupleDesc, isnull); }
void slot_getsomeattrs__pgrx_cshim(TupleTableSlot *slot, int attnum) { slot_getsomeattrs(slot, attnum); }
void slot_getallattrs__pgrx_cshim(TupleTableSlot *slot) { slot_getallattrs(slot); }
bool slot_attisnull__pgrx_cshim(TupleTableSlot *slot, int attnum) { return slot_attisnull(slot, attnum); }
Datum slot_getattr__pgrx_cshim(TupleTableSlot *slot, int attnum, bool *isnull) { return slot_getattr(slot, attnum, isnull); }
Datum slot_getsysattr__pgrx_cshim(TupleTableSlot *slot, int attnum, bool *isnull) { return slot_getsysattr(slot, attnum, isnull); }
TupleTableSlot * ExecClearTuple__pgrx_cshim(TupleTableSlot *slot) { return ExecClearTuple(slot); }
void ExecMaterializeSlot__pgrx_cshim(TupleTableSlot *slot) { ExecMaterializeSlot(slot); }
HeapTuple ExecCopySlotHeapTuple__pgrx_cshim(TupleTableSlot *slot) { return ExecCopySlotHeapTuple(slot); }
MinimalTuple ExecCopySlotMinimalTuple__pgrx_cshim(TupleTableSlot *slot) { return ExecCopySlotMinimalTuple(slot); }
TupleTableSlot * ExecCopySlot__pgrx_cshim(TupleTableSlot *dstslot, TupleTableSlot *srcslot) { return ExecCopySlot(dstslot, srcslot); }
void dlist_init__pgrx_cshim(dlist_head *head) { dlist_init(head); }
bool dlist_is_empty__pgrx_cshim(dlist_head *head) { return dlist_is_empty(head); }
void dlist_push_head__pgrx_cshim(dlist_head *head, dlist_node *node) { dlist_push_head(head, node); }
void dlist_push_tail__pgrx_cshim(dlist_head *head, dlist_node *node) { dlist_push_tail(head, node); }
void dlist_insert_after__pgrx_cshim(dlist_node *after, dlist_node *node) { dlist_insert_after(after, node); }
void dlist_insert_before__pgrx_cshim(dlist_node *before, dlist_node *node) { dlist_insert_before(before, node); }
void dlist_delete__pgrx_cshim(dlist_node *node) { dlist_delete(node); }
dlist_node * dlist_pop_head_node__pgrx_cshim(dlist_head *head) { return dlist_pop_head_node(head); }
void dlist_move_head__pgrx_cshim(dlist_head *head, dlist_node *node) { dlist_move_head(head, node); }
void dlist_move_tail__pgrx_cshim(dlist_head *head, dlist_node *node) { dlist_move_tail(head, node); }
bool dlist_has_next__pgrx_cshim(dlist_head *head, dlist_node *node) { return dlist_has_next(head, node); }
bool dlist_has_prev__pgrx_cshim(dlist_head *head, dlist_node *node) { return dlist_has_prev(head, node); }
dlist_node * dlist_next_node__pgrx_cshim(dlist_head *head, dlist_node *node) { return dlist_next_node(head, node); }
dlist_node * dlist_prev_node__pgrx_cshim(dlist_head *head, dlist_node *node) { return dlist_prev_node(head, node); }
void * dlist_head_element_off__pgrx_cshim(dlist_head *head, size_t off) { return dlist_head_element_off(head, off); }
dlist_node * dlist_head_node__pgrx_cshim(dlist_head *head) { return dlist_head_node(head); }
void * dlist_tail_element_off__pgrx_cshim(dlist_head *head, size_t off) { return dlist_tail_element_off(head, off); }
dlist_node * dlist_tail_node__pgrx_cshim(dlist_head *head) { return dlist_tail_node(head); }
void slist_init__pgrx_cshim(slist_head *head) { slist_init(head); }
bool slist_is_empty__pgrx_cshim(slist_head *head) { return slist_is_empty(head); }
void slist_push_head__pgrx_cshim(slist_head *head, slist_node *node) { slist_push_head(head, node); }
void slist_insert_after__pgrx_cshim(slist_node *after, slist_node *node) { slist_insert_after(after, node); }
slist_node * slist_pop_head_node__pgrx_cshim(slist_head *head) { return slist_pop_head_node(head); }
bool slist_has_next__pgrx_cshim(slist_head *head, slist_node *node) { return slist_has_next(head, node); }
slist_node * slist_next_node__pgrx_cshim(slist_head *head, slist_node *node) { return slist_next_node(head, node); }
void * slist_head_element_off__pgrx_cshim(slist_head *head, size_t off) { return slist_head_element_off(head, off); }
slist_node * slist_head_node__pgrx_cshim(slist_head *head) { return slist_head_node(head); }
void slist_delete_current__pgrx_cshim(slist_mutable_iter *iter) { slist_delete_current(iter); }
bool pg_atomic_test_set_flag_impl__pgrx_cshim(pg_atomic_flag *ptr) { return pg_atomic_test_set_flag_impl(ptr); }
bool pg_atomic_unlocked_test_flag_impl__pgrx_cshim(pg_atomic_flag *ptr) { return pg_atomic_unlocked_test_flag_impl(ptr); }
void pg_atomic_clear_flag_impl__pgrx_cshim(pg_atomic_flag *ptr) { pg_atomic_clear_flag_impl(ptr); }
void pg_atomic_init_flag_impl__pgrx_cshim(pg_atomic_flag *ptr) { pg_atomic_init_flag_impl(ptr); }
bool pg_atomic_compare_exchange_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 *expected, uint32 newval) { return pg_atomic_compare_exchange_u32_impl(ptr, expected, newval); }
uint32 pg_atomic_fetch_add_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, int32 add_) { return pg_atomic_fetch_add_u32_impl(ptr, add_); }
uint32 pg_atomic_fetch_sub_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, int32 sub_) { return pg_atomic_fetch_sub_u32_impl(ptr, sub_); }
uint32 pg_atomic_fetch_and_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 and_) { return pg_atomic_fetch_and_u32_impl(ptr, and_); }
uint32 pg_atomic_fetch_or_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 or_) { return pg_atomic_fetch_or_u32_impl(ptr, or_); }
bool pg_atomic_compare_exchange_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 *expected, uint64 newval) { return pg_atomic_compare_exchange_u64_impl(ptr, expected, newval); }
uint64 pg_atomic_fetch_add_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, int64 add_) { return pg_atomic_fetch_add_u64_impl(ptr, add_); }
uint64 pg_atomic_fetch_sub_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, int64 sub_) { return pg_atomic_fetch_sub_u64_impl(ptr, sub_); }
uint64 pg_atomic_fetch_and_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 and_) { return pg_atomic_fetch_and_u64_impl(ptr, and_); }
uint64 pg_atomic_fetch_or_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 or_) { return pg_atomic_fetch_or_u64_impl(ptr, or_); }
uint32 pg_atomic_read_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr) { return pg_atomic_read_u32_impl(ptr); }
void pg_atomic_write_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val) { pg_atomic_write_u32_impl(ptr, val); }
void pg_atomic_unlocked_write_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val) { pg_atomic_unlocked_write_u32_impl(ptr, val); }
void pg_atomic_init_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val_) { pg_atomic_init_u32_impl(ptr, val_); }
uint32 pg_atomic_exchange_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 xchg_) { return pg_atomic_exchange_u32_impl(ptr, xchg_); }
uint32 pg_atomic_add_fetch_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, int32 add_) { return pg_atomic_add_fetch_u32_impl(ptr, add_); }
uint32 pg_atomic_sub_fetch_u32_impl__pgrx_cshim(pg_atomic_uint32 *ptr, int32 sub_) { return pg_atomic_sub_fetch_u32_impl(ptr, sub_); }
uint64 pg_atomic_exchange_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 xchg_) { return pg_atomic_exchange_u64_impl(ptr, xchg_); }
void pg_atomic_write_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 val) { pg_atomic_write_u64_impl(ptr, val); }
uint64 pg_atomic_read_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr) { return pg_atomic_read_u64_impl(ptr); }
void pg_atomic_init_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 val_) { pg_atomic_init_u64_impl(ptr, val_); }
uint64 pg_atomic_add_fetch_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, int64 add_) { return pg_atomic_add_fetch_u64_impl(ptr, add_); }
uint64 pg_atomic_sub_fetch_u64_impl__pgrx_cshim(pg_atomic_uint64 *ptr, int64 sub_) { return pg_atomic_sub_fetch_u64_impl(ptr, sub_); }
void pg_atomic_init_flag__pgrx_cshim(pg_atomic_flag *ptr) { pg_atomic_init_flag(ptr); }
bool pg_atomic_test_set_flag__pgrx_cshim(pg_atomic_flag *ptr) { return pg_atomic_test_set_flag(ptr); }
bool pg_atomic_unlocked_test_flag__pgrx_cshim(pg_atomic_flag *ptr) { return pg_atomic_unlocked_test_flag(ptr); }
void pg_atomic_clear_flag__pgrx_cshim(pg_atomic_flag *ptr) { pg_atomic_clear_flag(ptr); }
void pg_atomic_init_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val) { pg_atomic_init_u32(ptr, val); }
uint32 pg_atomic_read_u32__pgrx_cshim(pg_atomic_uint32 *ptr) { return pg_atomic_read_u32(ptr); }
void pg_atomic_write_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val) { pg_atomic_write_u32(ptr, val); }
void pg_atomic_unlocked_write_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 val) { pg_atomic_unlocked_write_u32(ptr, val); }
uint32 pg_atomic_exchange_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 newval) { return pg_atomic_exchange_u32(ptr, newval); }
bool pg_atomic_compare_exchange_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 *expected, uint32 newval) { return pg_atomic_compare_exchange_u32(ptr, expected, newval); }
uint32 pg_atomic_fetch_add_u32__pgrx_cshim(pg_atomic_uint32 *ptr, int32 add_) { return pg_atomic_fetch_add_u32(ptr, add_); }
uint32 pg_atomic_fetch_sub_u32__pgrx_cshim(pg_atomic_uint32 *ptr, int32 sub_) { return pg_atomic_fetch_sub_u32(ptr, sub_); }
uint32 pg_atomic_fetch_and_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 and_) { return pg_atomic_fetch_and_u32(ptr, and_); }
uint32 pg_atomic_fetch_or_u32__pgrx_cshim(pg_atomic_uint32 *ptr, uint32 or_) { return pg_atomic_fetch_or_u32(ptr, or_); }
uint32 pg_atomic_add_fetch_u32__pgrx_cshim(pg_atomic_uint32 *ptr, int32 add_) { return pg_atomic_add_fetch_u32(ptr, add_); }
uint32 pg_atomic_sub_fetch_u32__pgrx_cshim(pg_atomic_uint32 *ptr, int32 sub_) { return pg_atomic_sub_fetch_u32(ptr, sub_); }
void pg_atomic_init_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 val) { pg_atomic_init_u64(ptr, val); }
uint64 pg_atomic_read_u64__pgrx_cshim(pg_atomic_uint64 *ptr) { return pg_atomic_read_u64(ptr); }
void pg_atomic_write_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 val) { pg_atomic_write_u64(ptr, val); }
uint64 pg_atomic_exchange_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 newval) { return pg_atomic_exchange_u64(ptr, newval); }
bool pg_atomic_compare_exchange_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 *expected, uint64 newval) { return pg_atomic_compare_exchange_u64(ptr, expected, newval); }
uint64 pg_atomic_fetch_add_u64__pgrx_cshim(pg_atomic_uint64 *ptr, int64 add_) { return pg_atomic_fetch_add_u64(ptr, add_); }
uint64 pg_atomic_fetch_sub_u64__pgrx_cshim(pg_atomic_uint64 *ptr, int64 sub_) { return pg_atomic_fetch_sub_u64(ptr, sub_); }
uint64 pg_atomic_fetch_and_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 and_) { return pg_atomic_fetch_and_u64(ptr, and_); }
uint64 pg_atomic_fetch_or_u64__pgrx_cshim(pg_atomic_uint64 *ptr, uint64 or_) { return pg_atomic_fetch_or_u64(ptr, or_); }
uint64 pg_atomic_add_fetch_u64__pgrx_cshim(pg_atomic_uint64 *ptr, int64 add_) { return pg_atomic_add_fetch_u64(ptr, add_); }
uint64 pg_atomic_sub_fetch_u64__pgrx_cshim(pg_atomic_uint64 *ptr, int64 sub_) { return pg_atomic_sub_fetch_u64(ptr, sub_); }
int tas__pgrx_cshim(slock_t *lock) { return tas(lock); }
void spin_delay__pgrx_cshim(void) { spin_delay(); }
void init_spin_delay__pgrx_cshim(SpinDelayStatus *status, const char *file, int line, const char *func) { init_spin_delay(status, file, line, func); }
int ApplySortComparator__pgrx_cshim(Datum datum1, bool isNull1, Datum datum2, bool isNull2, SortSupport ssup) { return ApplySortComparator(datum1, isNull1, datum2, isNull2, ssup); }
int ApplyUnsignedSortComparator__pgrx_cshim(Datum datum1, bool isNull1, Datum datum2, bool isNull2, SortSupport ssup) { return ApplyUnsignedSortComparator(datum1, isNull1, datum2, isNull2, ssup); }
int ApplySignedSortComparator__pgrx_cshim(Datum datum1, bool isNull1, Datum datum2, bool isNull2, SortSupport ssup) { return ApplySignedSortComparator(datum1, isNull1, datum2, isNull2, ssup); }
int ApplyInt32SortComparator__pgrx_cshim(Datum datum1, bool isNull1, Datum datum2, bool isNull2, SortSupport ssup) { return ApplyInt32SortComparator(datum1, isNull1, datum2, isNull2, ssup); }
int ApplySortAbbrevFullComparator__pgrx_cshim(Datum datum1, bool isNull1, Datum datum2, bool isNull2, SortSupport ssup) { return ApplySortAbbrevFullComparator(datum1, isNull1, datum2, isNull2, ssup); }
int pg_leftmost_one_pos32__pgrx_cshim(uint32 word) { return pg_leftmost_one_pos32(word); }
int pg_leftmost_one_pos64__pgrx_cshim(uint64 word) { return pg_leftmost_one_pos64(word); }
int pg_rightmost_one_pos32__pgrx_cshim(uint32 word) { return pg_rightmost_one_pos32(word); }
int pg_rightmost_one_pos64__pgrx_cshim(uint64 word) { return pg_rightmost_one_pos64(word); }
uint32 pg_nextpower2_32__pgrx_cshim(uint32 num) { return pg_nextpower2_32(num); }
uint64 pg_nextpower2_64__pgrx_cshim(uint64 num) { return pg_nextpower2_64(num); }
uint32 pg_prevpower2_32__pgrx_cshim(uint32 num) { return pg_prevpower2_32(num); }
uint64 pg_prevpower2_64__pgrx_cshim(uint64 num) { return pg_prevpower2_64(num); }
uint32 pg_ceil_log2_32__pgrx_cshim(uint32 num) { return pg_ceil_log2_32(num); }
uint64 pg_ceil_log2_64__pgrx_cshim(uint64 num) { return pg_ceil_log2_64(num); }
uint32 pg_rotate_right32__pgrx_cshim(uint32 word, int n) { return pg_rotate_right32(word, n); }
uint32 pg_rotate_left32__pgrx_cshim(uint32 word, int n) { return pg_rotate_left32(word, n); }
void SetQueryCompletion__pgrx_cshim(QueryCompletion *qc, CommandTag commandTag, uint64 nprocessed) { SetQueryCompletion(qc, commandTag, nprocessed); }
void CopyQueryCompletion__pgrx_cshim(QueryCompletion *dst, const QueryCompletion *src) { CopyQueryCompletion(dst, src); }
Datum ExecGetJunkAttribute__pgrx_cshim(TupleTableSlot *slot, AttrNumber attno, bool *isNull) { return ExecGetJunkAttribute(slot, attno, isNull); }
TupleTableSlot * ExecProcNode__pgrx_cshim(PlanState *node) { return ExecProcNode(node); }
Datum ExecEvalExpr__pgrx_cshim(ExprState *state, ExprContext *econtext, bool *isNull) { return ExecEvalExpr(state, econtext, isNull); }
Datum ExecEvalExprSwitchContext__pgrx_cshim(ExprState *state, ExprContext *econtext, bool *isNull) { return ExecEvalExprSwitchContext(state, econtext, isNull); }
TupleTableSlot * ExecProject__pgrx_cshim(ProjectionInfo *projInfo) { return ExecProject(projInfo); }
bool ExecQual__pgrx_cshim(ExprState *state, ExprContext *econtext) { return ExecQual(state, econtext); }
bool ExecQualAndReset__pgrx_cshim(ExprState *state, ExprContext *econtext) { return ExecQualAndReset(state, econtext); }
RangeTblEntry * exec_rt_fetch__pgrx_cshim(Index rti, EState *estate) { return exec_rt_fetch(rti, estate); }
bool is_unixsock_path__pgrx_cshim(const char *path) { return is_unixsock_path(path); }
void pgstat_report_wait_start__pgrx_cshim(uint32 wait_event_info) { pgstat_report_wait_start(wait_event_info); }
void pgstat_report_wait_end__pgrx_cshim(void) { pgstat_report_wait_end(); }
bool RmgrIdIsBuiltin__pgrx_cshim(int rmid) { return RmgrIdIsBuiltin(rmid); }
bool RmgrIdIsCustom__pgrx_cshim(int rmid) { return RmgrIdIsCustom(rmid); }
bool XLogReaderHasQueuedRecordOrError__pgrx_cshim(XLogReaderState *state) { return XLogReaderHasQueuedRecordOrError(state); }
bool RmgrIdExists__pgrx_cshim(RmgrId rmid) { return RmgrIdExists(rmid); }
RmgrData GetRmgr__pgrx_cshim(RmgrId rmid) { return GetRmgr(rmid); }
SMgrRelation RelationGetSmgr__pgrx_cshim(Relation rel) { return RelationGetSmgr(rel); }
void GistPageSetDeleted__pgrx_cshim(Page page, FullTransactionId deletexid) { GistPageSetDeleted(page, deletexid); }
FullTransactionId GistPageGetDeleteXid__pgrx_cshim(Page page) { return GistPageGetDeleteXid(page); }
TableScanDesc table_beginscan__pgrx_cshim(Relation rel, Snapshot snapshot, int nkeys, struct ScanKeyData *key) { return table_beginscan(rel, snapshot, nkeys, key); }
TableScanDesc table_beginscan_strat__pgrx_cshim(Relation rel, Snapshot snapshot, int nkeys, struct ScanKeyData *key, bool allow_strat, bool allow_sync) { return table_beginscan_strat(rel, snapshot, nkeys, key, allow_strat, allow_sync); }
TableScanDesc table_beginscan_bm__pgrx_cshim(Relation rel, Snapshot snapshot, int nkeys, struct ScanKeyData *key) { return table_beginscan_bm(rel, snapshot, nkeys, key); }
TableScanDesc table_beginscan_sampling__pgrx_cshim(Relation rel, Snapshot snapshot, int nkeys, struct ScanKeyData *key, bool allow_strat, bool allow_sync, bool allow_pagemode) { return table_beginscan_sampling(rel, snapshot, nkeys, key, allow_strat, allow_sync, allow_pagemode); }
TableScanDesc table_beginscan_tid__pgrx_cshim(Relation rel, Snapshot snapshot) { return table_beginscan_tid(rel, snapshot); }
TableScanDesc table_beginscan_analyze__pgrx_cshim(Relation rel) { return table_beginscan_analyze(rel); }
void table_endscan__pgrx_cshim(TableScanDesc scan) { table_endscan(scan); }
void table_rescan__pgrx_cshim(TableScanDesc scan, struct ScanKeyData *key) { table_rescan(scan, key); }
void table_rescan_set_params__pgrx_cshim(TableScanDesc scan, struct ScanKeyData *key, bool allow_strat, bool allow_sync, bool allow_pagemode) { table_rescan_set_params(scan, key, allow_strat, allow_sync, allow_pagemode); }
bool table_scan_getnextslot__pgrx_cshim(TableScanDesc sscan, ScanDirection direction, TupleTableSlot *slot) { return table_scan_getnextslot(sscan, direction, slot); }
TableScanDesc table_beginscan_tidrange__pgrx_cshim(Relation rel, Snapshot snapshot, ItemPointer mintid, ItemPointer maxtid) { return table_beginscan_tidrange(rel, snapshot, mintid, maxtid); }
void table_rescan_tidrange__pgrx_cshim(TableScanDesc sscan, ItemPointer mintid, ItemPointer maxtid) { table_rescan_tidrange(sscan, mintid, maxtid); }
bool table_scan_getnextslot_tidrange__pgrx_cshim(TableScanDesc sscan, ScanDirection direction, TupleTableSlot *slot) { return table_scan_getnextslot_tidrange(sscan, direction, slot); }
void table_parallelscan_reinitialize__pgrx_cshim(Relation rel, ParallelTableScanDesc pscan) { table_parallelscan_reinitialize(rel, pscan); }
IndexFetchTableData * table_index_fetch_begin__pgrx_cshim(Relation rel) { return table_index_fetch_begin(rel); }
void table_index_fetch_reset__pgrx_cshim(struct IndexFetchTableData *scan) { table_index_fetch_reset(scan); }
void table_index_fetch_end__pgrx_cshim(struct IndexFetchTableData *scan) { table_index_fetch_end(scan); }
bool table_index_fetch_tuple__pgrx_cshim(struct IndexFetchTableData *scan, ItemPointer tid, Snapshot snapshot, TupleTableSlot *slot, bool *call_again, bool *all_dead) { return table_index_fetch_tuple(scan, tid, snapshot, slot, call_again, all_dead); }
bool table_tuple_fetch_row_version__pgrx_cshim(Relation rel, ItemPointer tid, Snapshot snapshot, TupleTableSlot *slot) { return table_tuple_fetch_row_version(rel, tid, snapshot, slot); }
bool table_tuple_tid_valid__pgrx_cshim(TableScanDesc scan, ItemPointer tid) { return table_tuple_tid_valid(scan, tid); }
bool table_tuple_satisfies_snapshot__pgrx_cshim(Relation rel, TupleTableSlot *slot, Snapshot snapshot) { return table_tuple_satisfies_snapshot(rel, slot, snapshot); }
TransactionId table_index_delete_tuples__pgrx_cshim(Relation rel, TM_IndexDeleteOp *delstate) { return table_index_delete_tuples(rel, delstate); }
void table_tuple_insert__pgrx_cshim(Relation rel, TupleTableSlot *slot, CommandId cid, int options, struct BulkInsertStateData *bistate) { table_tuple_insert(rel, slot, cid, options, bistate); }
void table_tuple_insert_speculative__pgrx_cshim(Relation rel, TupleTableSlot *slot, CommandId cid, int options, struct BulkInsertStateData *bistate, uint32 specToken) { table_tuple_insert_speculative(rel, slot, cid, options, bistate, specToken); }
void table_tuple_complete_speculative__pgrx_cshim(Relation rel, TupleTableSlot *slot, uint32 specToken, bool succeeded) { table_tuple_complete_speculative(rel, slot, specToken, succeeded); }
void table_multi_insert__pgrx_cshim(Relation rel, TupleTableSlot **slots, int nslots, CommandId cid, int options, struct BulkInsertStateData *bistate) { table_multi_insert(rel, slots, nslots, cid, options, bistate); }
TM_Result table_tuple_delete__pgrx_cshim(Relation rel, ItemPointer tid, CommandId cid, Snapshot snapshot, Snapshot crosscheck, bool wait, TM_FailureData *tmfd, bool changingPart) { return table_tuple_delete(rel, tid, cid, snapshot, crosscheck, wait, tmfd, changingPart); }
TM_Result table_tuple_update__pgrx_cshim(Relation rel, ItemPointer otid, TupleTableSlot *slot, CommandId cid, Snapshot snapshot, Snapshot crosscheck, bool wait, TM_FailureData *tmfd, LockTupleMode *lockmode, bool *update_indexes) { return table_tuple_update(rel, otid, slot, cid, snapshot, crosscheck, wait, tmfd, lockmode, update_indexes); }
TM_Result table_tuple_lock__pgrx_cshim(Relation rel, ItemPointer tid, Snapshot snapshot, TupleTableSlot *slot, CommandId cid, LockTupleMode mode, LockWaitPolicy wait_policy, uint8 flags, TM_FailureData *tmfd) { return table_tuple_lock(rel, tid, snapshot, slot, cid, mode, wait_policy, flags, tmfd); }
void table_finish_bulk_insert__pgrx_cshim(Relation rel, int options) { table_finish_bulk_insert(rel, options); }
void table_relation_set_new_filenode__pgrx_cshim(Relation rel, const RelFileNode *newrnode, char persistence, TransactionId *freezeXid, MultiXactId *minmulti) { table_relation_set_new_filenode(rel, newrnode, persistence, freezeXid, minmulti); }
void table_relation_nontransactional_truncate__pgrx_cshim(Relation rel) { table_relation_nontransactional_truncate(rel); }
void table_relation_copy_data__pgrx_cshim(Relation rel, const RelFileNode *newrnode) { table_relation_copy_data(rel, newrnode); }
void table_relation_copy_for_cluster__pgrx_cshim(Relation OldTable, Relation NewTable, Relation OldIndex, bool use_sort, TransactionId OldestXmin, TransactionId *xid_cutoff, MultiXactId *multi_cutoff, double *num_tuples, double *tups_vacuumed, double *tups_recently_dead) { table_relation_copy_for_cluster(OldTable, NewTable, OldIndex, use_sort, OldestXmin, xid_cutoff, multi_cutoff, num_tuples, tups_vacuumed, tups_recently_dead); }
void table_relation_vacuum__pgrx_cshim(Relation rel, struct VacuumParams *params, BufferAccessStrategy bstrategy) { table_relation_vacuum(rel, params, bstrategy); }
bool table_scan_analyze_next_block__pgrx_cshim(TableScanDesc scan, BlockNumber blockno, BufferAccessStrategy bstrategy) { return table_scan_analyze_next_block(scan, blockno, bstrategy); }
bool table_scan_analyze_next_tuple__pgrx_cshim(TableScanDesc scan, TransactionId OldestXmin, double *liverows, double *deadrows, TupleTableSlot *slot) { return table_scan_analyze_next_tuple(scan, OldestXmin, liverows, deadrows, slot); }
double table_index_build_scan__pgrx_cshim(Relation table_rel, Relation index_rel, struct IndexInfo *index_info, bool allow_sync, bool progress, IndexBuildCallback callback, void *callback_state, TableScanDesc scan) { return table_index_build_scan(table_rel, index_rel, index_info, allow_sync, progress, callback, callback_state, scan); }
double table_index_build_range_scan__pgrx_cshim(Relation table_rel, Relation index_rel, struct IndexInfo *index_info, bool allow_sync, bool anyvisible, bool progress, BlockNumber start_blockno, BlockNumber numblocks, IndexBuildCallback callback, void *callback_state, TableScanDesc scan) { return table_index_build_range_scan(table_rel, index_rel, index_info, allow_sync, anyvisible, progress, start_blockno, numblocks, callback, callback_state, scan); }
void table_index_validate_scan__pgrx_cshim(Relation table_rel, Relation index_rel, struct IndexInfo *index_info, Snapshot snapshot, struct ValidateIndexState *state) { table_index_validate_scan(table_rel, index_rel, index_info, snapshot, state); }
uint64 table_relation_size__pgrx_cshim(Relation rel, ForkNumber forkNumber) { return table_relation_size(rel, forkNumber); }
bool table_relation_needs_toast_table__pgrx_cshim(Relation rel) { return table_relation_needs_toast_table(rel); }
Oid table_relation_toast_am__pgrx_cshim(Relation rel) { return table_relation_toast_am(rel); }
void table_relation_fetch_toast_slice__pgrx_cshim(Relation toastrel, Oid valueid, int32 attrsize, int32 sliceoffset, int32 slicelength, struct varlena *result) { table_relation_fetch_toast_slice(toastrel, valueid, attrsize, sliceoffset, slicelength, result); }
void table_relation_estimate_size__pgrx_cshim(Relation rel, int32 *attr_widths, BlockNumber *pages, double *tuples, double *allvisfrac) { table_relation_estimate_size(rel, attr_widths, pages, tuples, allvisfrac); }
bool table_scan_bitmap_next_block__pgrx_cshim(TableScanDesc scan, struct TBMIterateResult *tbmres) { return table_scan_bitmap_next_block(scan, tbmres); }
bool table_scan_bitmap_next_tuple__pgrx_cshim(TableScanDesc scan, struct TBMIterateResult *tbmres, TupleTableSlot *slot) { return table_scan_bitmap_next_tuple(scan, tbmres, slot); }
bool table_scan_sample_next_block__pgrx_cshim(TableScanDesc scan, struct SampleScanState *scanstate) { return table_scan_sample_next_block(scan, scanstate); }
bool table_scan_sample_next_tuple__pgrx_cshim(TableScanDesc scan, struct SampleScanState *scanstate, TupleTableSlot *slot) { return table_scan_sample_next_tuple(scan, scanstate, slot); }
bool OldSnapshotThresholdActive__pgrx_cshim(void) { return OldSnapshotThresholdActive(); }
void TestForOldSnapshot__pgrx_cshim(Snapshot snapshot, Relation relation, Page page) { TestForOldSnapshot(snapshot, relation, page); }
int64 itemptr_encode__pgrx_cshim(ItemPointer itemptr) { return itemptr_encode(itemptr); }
void itemptr_decode__pgrx_cshim(ItemPointer itemptr, int64 encoded) { itemptr_decode(itemptr, encoded); }
const char * collprovider_name__pgrx_cshim(char c) { return collprovider_name(c); }
bool is_valid_unicode_codepoint__pgrx_cshim(pg_wchar c) { return is_valid_unicode_codepoint(c); }
bool is_utf16_surrogate_first__pgrx_cshim(pg_wchar c) { return is_utf16_surrogate_first(c); }
bool is_utf16_surrogate_second__pgrx_cshim(pg_wchar c) { return is_utf16_surrogate_second(c); }
pg_wchar surrogate_pair_to_codepoint__pgrx_cshim(pg_wchar first, pg_wchar second) { return surrogate_pair_to_codepoint(first, second); }
void pq_writeint8__pgrx_cshim(StringInfoData *buf, uint8 i) { pq_writeint8(buf, i); }
void pq_writeint16__pgrx_cshim(StringInfoData *buf, uint16 i) { pq_writeint16(buf, i); }
void pq_writeint32__pgrx_cshim(StringInfoData *buf, uint32 i) { pq_writeint32(buf, i); }
void pq_writeint64__pgrx_cshim(StringInfoData *buf, uint64 i) { pq_writeint64(buf, i); }
void pq_writestring__pgrx_cshim(StringInfoData *buf, const char *str) { pq_writestring(buf, str); }
void pq_sendint8__pgrx_cshim(StringInfo buf, uint8 i) { pq_sendint8(buf, i); }
void pq_sendint16__pgrx_cshim(StringInfo buf, uint16 i) { pq_sendint16(buf, i); }
void pq_sendint32__pgrx_cshim(StringInfo buf, uint32 i) { pq_sendint32(buf, i); }
void pq_sendint64__pgrx_cshim(StringInfo buf, uint64 i) { pq_sendint64(buf, i); }
void pq_sendbyte__pgrx_cshim(StringInfo buf, uint8 byt) { pq_sendbyte(buf, byt); }
void pq_sendint__pgrx_cshim(StringInfo buf, uint32 i, int b) { pq_sendint(buf, i, b); }
bool is_funcclause__pgrx_cshim(const void *clause) { return is_funcclause(clause); }
bool is_opclause__pgrx_cshim(const void *clause) { return is_opclause(clause); }
Node * get_leftop__pgrx_cshim(const void *clause) { return get_leftop(clause); }
Node * get_rightop__pgrx_cshim(const void *clause) { return get_rightop(clause); }
bool is_andclause__pgrx_cshim(const void *clause) { return is_andclause(clause); }
bool is_orclause__pgrx_cshim(const void *clause) { return is_orclause(clause); }
bool is_notclause__pgrx_cshim(const void *clause) { return is_notclause(clause); }
Expr * get_notclausearg__pgrx_cshim(const void *notclause) { return get_notclausearg(notclause); }
const char * GetScanKeyword__pgrx_cshim(int n, const ScanKeywordList *keywords) { return GetScanKeyword(n, keywords); }
TupleDesc expanded_record_get_tupdesc__pgrx_cshim(ExpandedRecordHeader *erh) { return expanded_record_get_tupdesc(erh); }
Datum expanded_record_get_field__pgrx_cshim(ExpandedRecordHeader *erh, int fnumber, bool *isnull) { return expanded_record_get_field(erh, fnumber, isnull); }
void walrcv_clear_result__pgrx_cshim(WalRcvExecResult *walres) { walrcv_clear_result(walres); }
const char * CreateCommandName__pgrx_cshim(Node *parsetree) { return CreateCommandName(parsetree); }
float4 get_float4_infinity__pgrx_cshim(void) { return get_float4_infinity(); }
float8 get_float8_infinity__pgrx_cshim(void) { return get_float8_infinity(); }
float4 get_float4_nan__pgrx_cshim(void) { return get_float4_nan(); }
float8 get_float8_nan__pgrx_cshim(void) { return get_float8_nan(); }
float4 float4_pl__pgrx_cshim(const float4 val1, const float4 val2) { return float4_pl(val1, val2); }
float8 float8_pl__pgrx_cshim(const float8 val1, const float8 val2) { return float8_pl(val1, val2); }
float4 float4_mi__pgrx_cshim(const float4 val1, const float4 val2) { return float4_mi(val1, val2); }
float8 float8_mi__pgrx_cshim(const float8 val1, const float8 val2) { return float8_mi(val1, val2); }
float4 float4_mul__pgrx_cshim(const float4 val1, const float4 val2) { return float4_mul(val1, val2); }
float8 float8_mul__pgrx_cshim(const float8 val1, const float8 val2) { return float8_mul(val1, val2); }
float4 float4_div__pgrx_cshim(const float4 val1, const float4 val2) { return float4_div(val1, val2); }
float8 float8_div__pgrx_cshim(const float8 val1, const float8 val2) { return float8_div(val1, val2); }
bool float4_eq__pgrx_cshim(const float4 val1, const float4 val2) { return float4_eq(val1, val2); }
bool float8_eq__pgrx_cshim(const float8 val1, const float8 val2) { return float8_eq(val1, val2); }
bool float4_ne__pgrx_cshim(const float4 val1, const float4 val2) { return float4_ne(val1, val2); }
bool float8_ne__pgrx_cshim(const float8 val1, const float8 val2) { return float8_ne(val1, val2); }
bool float4_lt__pgrx_cshim(const float4 val1, const float4 val2) { return float4_lt(val1, val2); }
bool float8_lt__pgrx_cshim(const float8 val1, const float8 val2) { return float8_lt(val1, val2); }
bool float4_le__pgrx_cshim(const float4 val1, const float4 val2) { return float4_le(val1, val2); }
bool float8_le__pgrx_cshim(const float8 val1, const float8 val2) { return float8_le(val1, val2); }
bool float4_gt__pgrx_cshim(const float4 val1, const float4 val2) { return float4_gt(val1, val2); }
bool float8_gt__pgrx_cshim(const float8 val1, const float8 val2) { return float8_gt(val1, val2); }
bool float4_ge__pgrx_cshim(const float4 val1, const float4 val2) { return float4_ge(val1, val2); }
bool float8_ge__pgrx_cshim(const float8 val1, const float8 val2) { return float8_ge(val1, val2); }
float4 float4_min__pgrx_cshim(const float4 val1, const float4 val2) { return float4_min(val1, val2); }
float8 float8_min__pgrx_cshim(const float8 val1, const float8 val2) { return float8_min(val1, val2); }
float4 float4_max__pgrx_cshim(const float4 val1, const float4 val2) { return float4_max(val1, val2); }
float8 float8_max__pgrx_cshim(const float8 val1, const float8 val2) { return float8_max(val1, val2); }
bool FPeq__pgrx_cshim(double A, double B) { return FPeq(A, B); }
bool FPne__pgrx_cshim(double A, double B) { return FPne(A, B); }
bool FPlt__pgrx_cshim(double A, double B) { return FPlt(A, B); }
bool FPle__pgrx_cshim(double A, double B) { return FPle(A, B); }
bool FPgt__pgrx_cshim(double A, double B) { return FPgt(A, B); }
bool FPge__pgrx_cshim(double A, double B) { return FPge(A, B); }
