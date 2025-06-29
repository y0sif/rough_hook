/****************************************************************************
** Meta object code from reading C++ file 'movelist.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/movelist.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'movelist.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_MoveList_t {
    QByteArrayData data[19];
    char stringdata0[189];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_MoveList_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_MoveList_t qt_meta_stringdata_MoveList = {
    {
QT_MOC_LITERAL(0, 0, 8), // "MoveList"
QT_MOC_LITERAL(1, 9, 11), // "moveClicked"
QT_MOC_LITERAL(2, 21, 0), // ""
QT_MOC_LITERAL(3, 22, 3), // "num"
QT_MOC_LITERAL(4, 26, 7), // "keyLeft"
QT_MOC_LITERAL(5, 34, 14), // "commentClicked"
QT_MOC_LITERAL(6, 49, 7), // "comment"
QT_MOC_LITERAL(7, 57, 10), // "selectMove"
QT_MOC_LITERAL(8, 68, 7), // "moveNum"
QT_MOC_LITERAL(9, 76, 7), // "setMove"
QT_MOC_LITERAL(10, 84, 3), // "ply"
QT_MOC_LITERAL(11, 88, 18), // "Chess::GenericMove"
QT_MOC_LITERAL(12, 107, 4), // "move"
QT_MOC_LITERAL(13, 112, 9), // "sanString"
QT_MOC_LITERAL(14, 122, 10), // "onMoveMade"
QT_MOC_LITERAL(15, 133, 13), // "onLinkClicked"
QT_MOC_LITERAL(16, 147, 3), // "url"
QT_MOC_LITERAL(17, 151, 16), // "selectChosenMove"
QT_MOC_LITERAL(18, 168, 20) // "onContextMenuRequest"

    },
    "MoveList\0moveClicked\0\0num\0keyLeft\0"
    "commentClicked\0comment\0selectMove\0"
    "moveNum\0setMove\0ply\0Chess::GenericMove\0"
    "move\0sanString\0onMoveMade\0onLinkClicked\0"
    "url\0selectChosenMove\0onContextMenuRequest"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_MoveList[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
       8,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       2,       // signalCount

 // signals: name, argc, parameters, tag, flags
       1,    2,   54,    2, 0x06 /* Public */,
       5,    2,   59,    2, 0x06 /* Public */,

 // slots: name, argc, parameters, tag, flags
       7,    1,   64,    2, 0x0a /* Public */,
       9,    4,   67,    2, 0x0a /* Public */,
      14,    3,   76,    2, 0x08 /* Private */,
      15,    1,   83,    2, 0x08 /* Private */,
      17,    0,   86,    2, 0x08 /* Private */,
      18,    0,   87,    2, 0x08 /* Private */,

 // signals: parameters
    QMetaType::Void, QMetaType::Int, QMetaType::Bool,    3,    4,
    QMetaType::Void, QMetaType::Int, QMetaType::QString,    3,    6,

 // slots: parameters
    QMetaType::Bool, QMetaType::Int,    8,
    QMetaType::Void, QMetaType::Int, 0x80000000 | 11, QMetaType::QString, QMetaType::QString,   10,   12,   13,    6,
    QMetaType::Void, 0x80000000 | 11, QMetaType::QString, QMetaType::QString,   12,   13,    6,
    QMetaType::Void, QMetaType::QUrl,   16,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void MoveList::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<MoveList *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->moveClicked((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< bool(*)>(_a[2]))); break;
        case 1: _t->commentClicked((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< const QString(*)>(_a[2]))); break;
        case 2: { bool _r = _t->selectMove((*reinterpret_cast< int(*)>(_a[1])));
            if (_a[0]) *reinterpret_cast< bool*>(_a[0]) = std::move(_r); }  break;
        case 3: _t->setMove((*reinterpret_cast< int(*)>(_a[1])),(*reinterpret_cast< const Chess::GenericMove(*)>(_a[2])),(*reinterpret_cast< const QString(*)>(_a[3])),(*reinterpret_cast< const QString(*)>(_a[4]))); break;
        case 4: _t->onMoveMade((*reinterpret_cast< const Chess::GenericMove(*)>(_a[1])),(*reinterpret_cast< const QString(*)>(_a[2])),(*reinterpret_cast< const QString(*)>(_a[3]))); break;
        case 5: _t->onLinkClicked((*reinterpret_cast< const QUrl(*)>(_a[1]))); break;
        case 6: _t->selectChosenMove(); break;
        case 7: _t->onContextMenuRequest(); break;
        default: ;
        }
    } else if (_c == QMetaObject::IndexOfMethod) {
        int *result = reinterpret_cast<int *>(_a[0]);
        {
            using _t = void (MoveList::*)(int , bool );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&MoveList::moveClicked)) {
                *result = 0;
                return;
            }
        }
        {
            using _t = void (MoveList::*)(int , const QString & );
            if (*reinterpret_cast<_t *>(_a[1]) == static_cast<_t>(&MoveList::commentClicked)) {
                *result = 1;
                return;
            }
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject MoveList::staticMetaObject = { {
    QMetaObject::SuperData::link<QWidget::staticMetaObject>(),
    qt_meta_stringdata_MoveList.data,
    qt_meta_data_MoveList,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *MoveList::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *MoveList::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_MoveList.stringdata0))
        return static_cast<void*>(this);
    return QWidget::qt_metacast(_clname);
}

int MoveList::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QWidget::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 8)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 8;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 8)
            *reinterpret_cast<int*>(_a[0]) = -1;
        _id -= 8;
    }
    return _id;
}

// SIGNAL 0
void MoveList::moveClicked(int _t1, bool _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 0, _a);
}

// SIGNAL 1
void MoveList::commentClicked(int _t1, const QString & _t2)
{
    void *_a[] = { nullptr, const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t1))), const_cast<void*>(reinterpret_cast<const void*>(std::addressof(_t2))) };
    QMetaObject::activate(this, &staticMetaObject, 1, _a);
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
