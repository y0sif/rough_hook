/****************************************************************************
** Meta object code from reading C++ file 'gamedatabasedlg.h'
**
** Created by: The Qt Meta Object Compiler version 67 (Qt 5.15.13)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include <memory>
#include "../../../projects/gui/src/gamedatabasedlg.h"
#include <QtCore/qbytearray.h>
#include <QtCore/qmetatype.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'gamedatabasedlg.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 67
#error "This file was generated using the moc from 5.15.13. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

QT_BEGIN_MOC_NAMESPACE
QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
struct qt_meta_stringdata_GameDatabaseDialog_t {
    QByteArrayData data[20];
    char stringdata0[245];
};
#define QT_MOC_LITERAL(idx, ofs, len) \
    Q_STATIC_BYTE_ARRAY_DATA_HEADER_INITIALIZER_WITH_OFFSET(len, \
    qptrdiff(offsetof(qt_meta_stringdata_GameDatabaseDialog_t, stringdata0) + ofs \
        - idx * sizeof(QByteArrayData)) \
    )
static const qt_meta_stringdata_GameDatabaseDialog_t qt_meta_stringdata_GameDatabaseDialog = {
    {
QT_MOC_LITERAL(0, 0, 18), // "GameDatabaseDialog"
QT_MOC_LITERAL(1, 19, 24), // "databaseSelectionChanged"
QT_MOC_LITERAL(2, 44, 0), // ""
QT_MOC_LITERAL(3, 45, 14), // "QItemSelection"
QT_MOC_LITERAL(4, 60, 8), // "selected"
QT_MOC_LITERAL(5, 69, 10), // "deselected"
QT_MOC_LITERAL(6, 80, 20), // "gameSelectionChanged"
QT_MOC_LITERAL(7, 101, 11), // "QModelIndex"
QT_MOC_LITERAL(8, 113, 7), // "current"
QT_MOC_LITERAL(9, 121, 8), // "previous"
QT_MOC_LITERAL(10, 130, 12), // "updateSearch"
QT_MOC_LITERAL(11, 143, 5), // "terms"
QT_MOC_LITERAL(12, 149, 15), // "onSearchTimeout"
QT_MOC_LITERAL(13, 165, 16), // "onAdvancedSearch"
QT_MOC_LITERAL(14, 182, 9), // "exportPgn"
QT_MOC_LITERAL(15, 192, 8), // "filename"
QT_MOC_LITERAL(16, 201, 17), // "createOpeningBook"
QT_MOC_LITERAL(17, 219, 8), // "copyGame"
QT_MOC_LITERAL(18, 228, 7), // "copyFen"
QT_MOC_LITERAL(19, 236, 8) // "updateUi"

    },
    "GameDatabaseDialog\0databaseSelectionChanged\0"
    "\0QItemSelection\0selected\0deselected\0"
    "gameSelectionChanged\0QModelIndex\0"
    "current\0previous\0updateSearch\0terms\0"
    "onSearchTimeout\0onAdvancedSearch\0"
    "exportPgn\0filename\0createOpeningBook\0"
    "copyGame\0copyFen\0updateUi"
};
#undef QT_MOC_LITERAL

static const uint qt_meta_data_GameDatabaseDialog[] = {

 // content:
       8,       // revision
       0,       // classname
       0,    0, // classinfo
      11,   14, // methods
       0,    0, // properties
       0,    0, // enums/sets
       0,    0, // constructors
       0,       // flags
       0,       // signalCount

 // slots: name, argc, parameters, tag, flags
       1,    2,   69,    2, 0x08 /* Private */,
       6,    2,   74,    2, 0x08 /* Private */,
      10,    1,   79,    2, 0x08 /* Private */,
      10,    0,   82,    2, 0x28 /* Private | MethodCloned */,
      12,    0,   83,    2, 0x08 /* Private */,
      13,    0,   84,    2, 0x08 /* Private */,
      14,    1,   85,    2, 0x08 /* Private */,
      16,    0,   88,    2, 0x08 /* Private */,
      17,    0,   89,    2, 0x08 /* Private */,
      18,    0,   90,    2, 0x08 /* Private */,
      19,    0,   91,    2, 0x08 /* Private */,

 // slots: parameters
    QMetaType::Void, 0x80000000 | 3, 0x80000000 | 3,    4,    5,
    QMetaType::Void, 0x80000000 | 7, 0x80000000 | 7,    8,    9,
    QMetaType::Void, QMetaType::QString,   11,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void, QMetaType::QString,   15,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,
    QMetaType::Void,

       0        // eod
};

void GameDatabaseDialog::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    if (_c == QMetaObject::InvokeMetaMethod) {
        auto *_t = static_cast<GameDatabaseDialog *>(_o);
        (void)_t;
        switch (_id) {
        case 0: _t->databaseSelectionChanged((*reinterpret_cast< const QItemSelection(*)>(_a[1])),(*reinterpret_cast< const QItemSelection(*)>(_a[2]))); break;
        case 1: _t->gameSelectionChanged((*reinterpret_cast< const QModelIndex(*)>(_a[1])),(*reinterpret_cast< const QModelIndex(*)>(_a[2]))); break;
        case 2: _t->updateSearch((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 3: _t->updateSearch(); break;
        case 4: _t->onSearchTimeout(); break;
        case 5: _t->onAdvancedSearch(); break;
        case 6: _t->exportPgn((*reinterpret_cast< const QString(*)>(_a[1]))); break;
        case 7: _t->createOpeningBook(); break;
        case 8: _t->copyGame(); break;
        case 9: _t->copyFen(); break;
        case 10: _t->updateUi(); break;
        default: ;
        }
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        switch (_id) {
        default: *reinterpret_cast<int*>(_a[0]) = -1; break;
        case 0:
            switch (*reinterpret_cast<int*>(_a[1])) {
            default: *reinterpret_cast<int*>(_a[0]) = -1; break;
            case 1:
            case 0:
                *reinterpret_cast<int*>(_a[0]) = qRegisterMetaType< QItemSelection >(); break;
            }
            break;
        }
    }
}

QT_INIT_METAOBJECT const QMetaObject GameDatabaseDialog::staticMetaObject = { {
    QMetaObject::SuperData::link<QDialog::staticMetaObject>(),
    qt_meta_stringdata_GameDatabaseDialog.data,
    qt_meta_data_GameDatabaseDialog,
    qt_static_metacall,
    nullptr,
    nullptr
} };


const QMetaObject *GameDatabaseDialog::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *GameDatabaseDialog::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_meta_stringdata_GameDatabaseDialog.stringdata0))
        return static_cast<void*>(this);
    return QDialog::qt_metacast(_clname);
}

int GameDatabaseDialog::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QDialog::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 11)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 11;
    } else if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 11)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 11;
    }
    return _id;
}
QT_WARNING_POP
QT_END_MOC_NAMESPACE
